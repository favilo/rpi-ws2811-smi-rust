use std::{thread, time::Duration};

use crate::{
    error::Error, utils::gpio_mode, MemMap, GPIO_ALT1, LED_D0_PIN, LED_NCHANS, REQUEST_THRESH,
};

pub(crate) const SMI_8_BITS: usize = 0;
pub(crate) const SMI_16_BITS: usize = 1;
pub(crate) const SMI_18_BITS: usize = 2;
pub(crate) const SMI_9_BITS: usize = 3;

// TODO: Enable fields
#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiCsReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiLReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiAReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiDReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiDmcReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiDsrReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiDswReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiDcsReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiDcaReg(pub *mut u32);

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SmiDcdReg(pub *mut u32);

#[derive(Debug)]
pub(crate) struct Smi {
    pub cs: SmiCsReg,
    pub l: SmiLReg,
    pub a: SmiAReg,
    pub d: SmiDReg,
    pub dmc: SmiDmcReg,
    pub dsr: SmiDsrReg,
    pub dsw: SmiDswReg,
    pub dcs: SmiDcsReg,
    pub dca: SmiDcaReg,
    pub dcd: SmiDcdReg,
}

impl Default for Smi {
    fn default() -> Self {
        Self {
            cs: SmiCsReg(std::ptr::null_mut()),
            l: SmiLReg(std::ptr::null_mut()),
            a: SmiAReg(std::ptr::null_mut()),
            d: SmiDReg(std::ptr::null_mut()),
            dmc: SmiDmcReg(std::ptr::null_mut()),
            dsr: SmiDsrReg(std::ptr::null_mut()),
            dsw: SmiDswReg(std::ptr::null_mut()),
            dcs: SmiDcsReg(std::ptr::null_mut()),
            dca: SmiDcaReg(std::ptr::null_mut()),
            dcd: SmiDcdReg(std::ptr::null_mut()),
        }
    }
}

macro_rules! REG32 {
    ($name: ident, $offset: expr) => {
        (unsafe {
            $name
                .virt
                .as_ref()
                .ok_or(Error::Uninitialized)?
                .as_ptr()
                .offset($offset)
        }) as *mut u32
    };
}

macro_rules! set_bits {
    ($name: expr, $value: expr, $bits: expr, $offset: expr) => {
        $name
            .write_volatile($name.read_volatile() | ($value as u32 & ((1 << $bits) - 1)) << $offset)
    };
}

pub(crate) use {set_bits, REG32};

const CLK_SMI_CTL: isize = 0xB0;
const CLK_SMI_DIV: isize = 0xB4;
const CLK_PASSWD: u32 = 0x5a000000;

impl Smi {
    pub(crate) unsafe fn init(
        &mut self,
        smi_regs: &MemMap,
        clk_regs: &MemMap,
        gpio_regs: &MemMap,
        width: usize,
        timings: [u8; 4],
    ) -> Result<(), Error> {
        let [ns, setup, strobe, hold] = timings;
        let divi = (ns >> 1) as u32;
        self.cs = SmiCsReg(REG32!(smi_regs, 0x00));
        self.l = SmiLReg(REG32!(smi_regs, 0x04));
        self.a = SmiAReg(REG32!(smi_regs, 0x08));
        self.d = SmiDReg(REG32!(smi_regs, 0x0C));
        self.dmc = SmiDmcReg(REG32!(smi_regs, 0x30));
        self.dsr = SmiDsrReg(REG32!(smi_regs, 0x10));
        self.dsw = SmiDswReg(REG32!(smi_regs, 0x14));
        self.dcs = SmiDcsReg(REG32!(smi_regs, 0x34));
        self.dca = SmiDcaReg(REG32!(smi_regs, 0x38));
        self.dcd = SmiDcdReg(REG32!(smi_regs, 0x3C));

        self.cs.0.write_volatile(0);
        self.l.0.write_volatile(0);
        self.a.0.write_volatile(0);
        self.dsr.0.write_volatile(0);
        self.dsw.0.write_volatile(0);
        self.dcs.0.write_volatile(0);
        self.dca.0.write_volatile(0);

        if REG32!(clk_regs, CLK_SMI_DIV).read_volatile() != divi << 12 {
            REG32!(clk_regs, CLK_SMI_CTL).write_volatile(CLK_PASSWD | (1 << 5));
            thread::sleep(Duration::from_micros(10));
            // Busy wait for clock to be ready
            while REG32!(clk_regs, CLK_SMI_CTL).read_volatile() & (1 << 7) != 0 {}
            thread::sleep(Duration::from_micros(10));
            REG32!(clk_regs, CLK_SMI_DIV).write_volatile(CLK_PASSWD | (divi << 12));
            thread::sleep(Duration::from_micros(10));
            REG32!(clk_regs, CLK_SMI_CTL).write_volatile(CLK_PASSWD | 6 | (1 << 4));
            thread::sleep(Duration::from_micros(10));
            while REG32!(clk_regs, CLK_SMI_CTL).read_volatile() & (1 << 7) == 0 {}
            thread::sleep(Duration::from_micros(100));
        }
        // if (smi_cs->seterr)
        if (self.cs.0.read_volatile() >> 16) & 1 != 0 {
            // smi_cs->seterr = 1;
            self.cs
                .0
                .write_volatile(self.cs.0.read_volatile() | (1 << 16));
        }
        // smi_dsr->rsetup = smi_dsw->wsetup = setup;
        set_bits!(self.dsr.0, setup, 6, 2);
        set_bits!(self.dsw.0, setup, 6, 2);
        // smi_dsr->rstrobe = smi_dsw->wstrobe = strobe;
        set_bits!(self.dsr.0, strobe, 7, 24);
        set_bits!(self.dsw.0, strobe, 7, 24);
        // smi_dsr->rhold = smi_dsw->whold = hold;
        set_bits!(self.dsr.0, hold, 6, 9);
        set_bits!(self.dsw.0, hold, 6, 9);
        // smi_dmc->panicr = smi_dmc->panicw = 8;
        set_bits!(self.dmc.0, 8, 6, 5);
        set_bits!(self.dmc.0, 8, 6, 11);
        // smi_dmc->reqr = smi_dmc->reqw = REQUEST_THRESH;
        set_bits!(self.dmc.0, REQUEST_THRESH, 6, 17);
        set_bits!(self.dmc.0, REQUEST_THRESH, 6, 23);
        // smi_dsr->rwidth = smi_dsw->wwidth = width;
        set_bits!(self.dsr.0, width, 2, 0);
        set_bits!(self.dsw.0, width, 2, 0);

        // for (i = 0; i < LED_NCHANS; i++)
        for i in 0..LED_NCHANS as u8 {
            unsafe {
                gpio_mode(gpio_regs, LED_D0_PIN + i, GPIO_ALT1)?;
            }
        }
        Ok(())
    }
}
