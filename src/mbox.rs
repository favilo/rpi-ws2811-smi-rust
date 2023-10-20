use std::{fs::File, os::fd::AsRawFd};

use memmap2::MmapRaw;
use nix::request_code_readwrite;

use crate::{
    dma::{
        DmaCb, VcAllocFlags, BUS_PHYS_ADDR, DMA_CB_SRCE_INC, DMA_CHAN, DMA_CONBLK_AD, DMA_CS,
        DMA_DEBUG, DMA_DEST_DREQ, DMA_ENABLE, DMA_MEM_FLAGS, DMA_REG, DMA_SMI_DREQ, DMA_WAIT_RESP,
        MEM_BUS_ADDR, REG_BUS_ADDR,
    },
    error::Error,
    smi::{Smi, REG32},
    utils::{map_segment, page_roundup},
    MemMap, PAGE_SIZE,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct Msg {
    len: u32,             // Overall length (bytes)
    req: u32,             // Zero for request, 1<<31 for response
    tag: u32,             // Command number
    blen: u32,            // Buffer length (bytes)
    dlen: u32,            // Data length (bytes)
    uints: [u32; 32 - 5], // Data (108 bytes maximum)
}

#[derive(Debug)]
pub(crate) struct VcMap {
    file: File,
    h: u32,
    size: u32,
    bus: *mut c_void,
    virt: Option<MmapRaw>,
}

impl VcMap {
    pub(crate) fn map_uncached_mem(size: u32) -> Result<Self, Error> {
        // void *ret;
        // mp->size = PAGE_ROUNDUP(size);
        let size = page_roundup(size);
        // mp->fd = open_mbox();
        let file = open_mbox()?;
        // ret = (mp->h = alloc_vc_mem(mp->fd, mp->size, DMA_MEM_FLAGS)) > 0 &&
        //     (mp->bus = lock_vc_mem(mp->fd, mp->h)) != 0 &&
        //     (mp->virt = map_segment(BUS_PHYS_ADDR(mp->bus), mp->size)) != 0
        //     ? mp->virt : 0;
        // printf("VC mem handle %u, phys %p, virt %p\n", mp->h, mp->bus, mp->virt);
        // return(ret);

        let mut this = Self {
            file,
            h: 0,
            size,
            bus: std::ptr::null_mut(),
            virt: None,
        };

        this.alloc_vc_mem(DMA_MEM_FLAGS)?;
        this.lock_vc_mem()?;
        this.virt = Some(map_segment(BUS_PHYS_ADDR!(this.bus), this.size)?);
        log::info!(
            "VC mem handle {}, phys {:?}, virt {:?}",
            this.h,
            this.bus,
            this.virt.as_ref().unwrap().as_ptr()
        );
        Ok(this)
    }

    fn msg_mbox(&self, msg: Msg) -> Result<Msg, Error> {
        let mut ret = msg.clone();
        ret.len = (msg.len + 6) * 4;
        ret.req = 0;
        let code = request_code_readwrite!(100, 0, std::mem::size_of::<*mut c_void>());
        let fd = self.file.as_raw_fd();
        if unsafe { libc::ioctl(fd, code, &mut ret) } < 0 {
            log::error!("VC IOCTL failed");
            return Err(Error::VideoCoreFailed);
        } else if ret.req & 0x80000000 == 0 {
            log::error!("VC IOCTL error");
            return Err(Error::VideoCoreError);
        } else if ret.req == 0x80000001 {
            log::error!("VC IOCTL partial error");
            return Err(Error::VideoCorePartialError);
        }

        log::debug!(
            "VC msg len={}, req={:0X}, tag={:0X}, blen={:0x}, dlen={:0x}, data {:#?}",
            ret.len,
            ret.req,
            ret.tag,
            ret.blen,
            ret.dlen,
            ret.uints
        );
        Ok(ret)
    }

    fn alloc_vc_mem(&mut self, flags: VcAllocFlags) -> Result<(), Error> {
        let mut uints = [0u32; 32 - 5];
        uints[0] = page_roundup(self.size) as u32;
        uints[1] = PAGE_SIZE as u32;
        uints[2] = flags.0 as u32;
        let msg = Msg {
            tag: 0x30000C,
            blen: 12,
            dlen: 12,
            uints,
            ..Default::default()
        };
        self.h = self.msg_mbox(msg)?.uints[0];
        Ok(())
    }

    fn lock_vc_mem(&mut self) -> Result<(), Error> {
        if self.h == 0 {
            return Err(Error::VideoCoreFailed);
        }
        let mut uints = [0u32; 32 - 5];
        uints[0] = self.h;
        let msg = Msg {
            tag: 0x30000D,
            blen: 4,
            dlen: 4,
            uints,
            ..Default::default()
        };
        self.bus = self.msg_mbox(msg)?.uints[0] as *mut c_void;
        Ok(())
    }

    pub(crate) unsafe fn setup_smi_dma(
        &self,
        txdata: &mut *mut u16,
        smi: &Smi,
        smi_regs: &MemMap,
        dma_regs: &MemMap,
        n_samp: usize,
    ) -> Result<(), Error> {
        // DMA_CB *cbs = mp->virt;
        let cbs: *mut DmaCb = self
            .virt
            .as_ref()
            .ok_or(Error::VcMemUninitialized)?
            .as_mut_ptr() as *mut DmaCb;

        // txdata = (TXDATA_T * )(cbs + 1);
        *txdata = unsafe { (cbs as *mut u16).offset(1) };
        unsafe {
            // smi_dmc->dmaen = 1;
            smi.dmc.0.write_volatile(smi.dmc.0.read_volatile() | 0x01);
            // smi_cs->enable = 1;
            smi.cs
                .0
                .write_volatile(smi.cs.0.read_volatile() | 0x01 << 31);
            // smi_cs->clear = 1;
            smi.cs
                .0
                .write_volatile(smi.cs.0.read_volatile() | 0x01 << 27);
            // smi_cs->pxldat = 1;
            smi.cs
                .0
                .write_volatile(smi.cs.0.read_volatile() | 0x01 << 17);
            // smi_l->len = nsamp * sizeof(TXDATA_T);
            smi.l
                .0
                .write_volatile(n_samp as u32 * std::mem::size_of::<u16>() as u32);
            // smi_cs->write = 1;
            smi.cs
                .0
                .write_volatile(smi.cs.0.read_volatile() | 0x01 << 26);
            enable_dma(dma_regs, DMA_CHAN as u32)?;

            // cbs[0].ti = DMA_DEST_DREQ | (DMA_SMI_DREQ << 16) | DMA_CB_SRCE_INC | DMA_WAIT_RESP;
            (*cbs).ti = DMA_DEST_DREQ | DMA_SMI_DREQ << 16 | DMA_CB_SRCE_INC | DMA_WAIT_RESP;

            // cbs[0].tfr_len = nsamp * sizeof(TXDATA_T);
            (*cbs).tfr_len = n_samp as u32 * std::mem::size_of::<u16>() as u32;

            // cbs[0].srce_ad = MEM_BUS_ADDR(mp, txdata);
            (*cbs).srce_ad = BUS_PHYS_ADDR!(*txdata) as u32;

            // cbs[0].dest_ad = REG_BUS_ADDR(smi_regs, SMI_D);
            (*cbs).dest_ad = REG_BUS_ADDR!(smi_regs, 0x0c) as u32;
        };

        Ok(())
    }

    pub(crate) unsafe fn start_smi(&self, smi: &Smi) -> Result<(), Error> {
        let cbs = self
            .virt
            .as_ref()
            .ok_or(Error::SmiUninitialized)?
            .as_mut_ptr() as *mut DmaCb;

        unsafe {
            // start_dma(mp, DMA_CHAN, &cbs[0], 0);
            start_dma(&self, DMA_CHAN, cbs, 0)?;
            // smi_cs->start = 1;
            smi.cs.0.write_volatile(smi.cs.0.read_volatile() | 1 << 28);
        }
        Ok(())
    }
}

unsafe fn enable_dma(dma_regs: &MemMap, chan: u32) -> Result<(), Error> {
    // *REG32(dma_regs, DMA_ENABLE) |= (1 << chan);
    let dma_enabled = REG32!(dma_regs, DMA_ENABLE as isize);
    unsafe { dma_enabled.write_volatile(dma_enabled.read_volatile() | (1 << chan)) };
    // *REG32(dma_regs, DMA_REG(chan, DMA_CS)) = 1 << 31;
    REG32!(dma_regs, DMA_REG!(chan as usize, DMA_CS) as isize).write_volatile(1 << 31);

    Ok(())
}

unsafe fn start_dma(mp: &VcMap, chan: usize, cbp: *const DmaCb, csval: u32) -> Result<(), Error> {
    // *REG32(dma_regs, DMA_REG(chan, DMA_CONBLK_AD)) = MEM_BUS_ADDR(mp, cbp);
    let dma_conblk_ad = REG32!(mp, DMA_REG!(chan, DMA_CONBLK_AD) as isize);
    dma_conblk_ad.write_volatile(MEM_BUS_ADDR!(mp, cbp) as u32);
    // *REG32(dma_regs, DMA_REG(chan, DMA_CS)) = 2;        // Clear 'end' flag
    let dma_cs = REG32!(mp, DMA_REG!(chan, DMA_CS) as isize);
    dma_cs.write_volatile(2);
    // *REG32(dma_regs, DMA_REG(chan, DMA_DEBUG)) = 7;     // Clear error bits
    let dma_debug = REG32!(mp, DMA_REG!(chan, DMA_DEBUG) as isize);
    dma_debug.write_volatile(7);
    // *REG32(dma_regs, DMA_REG(chan, DMA_CS)) = 1|csval;  // Start DMA
    dma_cs.write_volatile(1 | csval);
    Ok(())
}

pub(crate) fn open_mbox() -> Result<File, Error> {
    Ok(File::open("/dev/vcio")?)
}
