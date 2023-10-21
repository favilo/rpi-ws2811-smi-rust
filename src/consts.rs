use libc::c_void;

#[cfg(rpi4)]
pub(crate) const SMI_TIMING: [u8; 4] = [10, 15, 30, 15];
#[cfg(not(rpi4))]
pub(crate) const SMI_TIMING: [u8; 4] = [10, 10, 20, 10];

/// GPIO pin for D0 output
pub(crate) const LED_D0_PIN: u8 = 8;
/// Number of LED channels (8 or 16)
pub(crate) const LED_NCHANS: usize = 16;
/// Number of bits per LED
pub(crate) const LED_NBITS: usize = 24;
/// Number of zero bits before LED data
pub(crate) const LED_PREBITS: usize = 4;
/// Number of zero bits after LED data
pub(crate) const LED_POSTBITS: usize = 4;
/// Number of O/P pulses per LED bit
pub(crate) const BIT_NPULSES: usize = 3;
/// Max number of LEDs per channel
pub(crate) const CHAN_MAXLEDS: usize = 512;
/// DMA request threshold
pub(crate) const REQUEST_THRESH: usize = 2;

pub(crate) const GPIO_ALT1: u8 = 5;

pub(crate) const LED_DLEN: usize = LED_NBITS * BIT_NPULSES;

#[cfg(rpi4)]
pub(crate) const PHYS_REG_BASE: *mut c_void = 0xFE000000 as *mut c_void;
#[cfg(rpi23)]
pub(crate) const PHYS_REG_BASE: *mut c_void = 0x3F000000 as *mut c_void;
#[cfg(not(any(rpi23, rpi4)))]
pub(crate) const PHYS_REG_BASE: *const c_void = 0x20000000 as *const c_void;

pub(crate) const GPIO_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x200000);
pub(crate) const DMA_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x007000);
pub(crate) const CLK_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x101000);
pub(crate) const SMI_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x600000);
pub(crate) const BUS_REG_BASE: *mut c_void = 0x7E000000 as *mut c_void;
pub(crate) const PAGE_SIZE: usize = 0x1000;

macro_rules! LED_TX_OFFSET {
    ($n: expr) => {
        LED_PREBITS + (LED_DLEN * $n)
    };
}

macro_rules! TX_BUFF_LEN {
    ($n: expr) => {
        LED_TX_OFFSET!($n) + LED_POSTBITS
    };
}

macro_rules! TX_BUFF_SIZE {
    ($n: expr) => {
        TX_BUFF_LEN!($n) * std::mem::size_of::<u16>()
    };
}

pub(crate) use {LED_TX_OFFSET, TX_BUFF_LEN, TX_BUFF_SIZE};

pub(crate) const VC_MEM_SIZE: usize = PAGE_SIZE + TX_BUFF_SIZE!(CHAN_MAXLEDS);
