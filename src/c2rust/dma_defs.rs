use libc::c_void;

use crate::consts::PHYS_REG_BASE;

// DMA channels and data requests
pub(crate) const DMA_CHAN_A: usize = 10;
pub(crate) const DMA_CHAN_B: usize = 11;
pub(crate) const DMA_PWM_DREQ: usize = 5;
pub(crate) const DMA_SPI_TX_DREQ: usize = 6;
pub(crate) const DMA_SPI_RX_DREQ: usize = 7;
pub(crate) const DMA_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x007000);
// DMA register addresses offset by 0x100 * chan_num
pub(crate) const DMA_CS: usize = 0x00;
pub(crate) const DMA_CONBLK_AD: usize = 0x04;
pub(crate) const DMA_TI: usize = 0x08;
pub(crate) const DMA_SRCE_AD: usize = 0x0c;
pub(crate) const DMA_DEST_AD: usize = 0x10;
pub(crate) const DMA_TXFR_LEN: usize = 0x14;
pub(crate) const DMA_STRIDE: usize = 0x18;
pub(crate) const DMA_NEXTCONBK: usize = 0x1c;
pub(crate) const DMA_DEBUG: usize = 0x20;
pub(crate) const DMA_ENABLE: usize = 0xff0;
// #define DMA_REG(ch, r)  ((r)==DMA_ENABLE ? DMA_ENABLE : (ch)*0x100+(r))

// // DMA register values
pub(crate) const DMA_WAIT_RESP: usize = 1 << 3;
pub(crate) const DMA_CB_DEST_INC: usize = 1 << 4;
pub(crate) const DMA_DEST_DREQ: usize = 1 << 6;
pub(crate) const DMA_CB_SRCE_INC: usize = 1 << 8;
pub(crate) const DMA_SRCE_DREQ: usize = 1 << 10;
// #define DMA_PRIORITY(n) ((n) << 16)

pub(crate) const DMA_SMI_DREQ: usize = 4;

macro_rules! DMA_REG {
    ($chan:expr, $reg:expr) => {
        if $reg == DMA_ENABLE {
            DMA_ENABLE
        } else {
            $chan * 0x100 + $reg
        }
    };
}

macro_rules! DMA_PRIORITY {
    ($n:expr) => {
        $n << 16
    };
}

pub(crate) use {DMA_PRIORITY, DMA_REG};

macro_rules! MEM_BUS_ADDR {
    ($mp: expr, $a: expr) => {
        ($a as u32)
            .wrapping_sub($mp.virt as u32)
            .wrapping_add($mp.bus as u32)
    };
}

macro_rules! BUS_PHYS_ADDR {
    ($a: expr) => {
        ($a as usize & !0xC0000000) as *mut std::ffi::c_void
    };
}

macro_rules! REG_BUS_ADDR {
    ($m: expr, $x: expr) => {
        ($m.bus as usize + $x) as *mut std::ffi::c_void
    };
}

pub(crate) use {BUS_PHYS_ADDR, MEM_BUS_ADDR, REG_BUS_ADDR};
