pub(crate) enum VcAllocFlagsEnum {
    Discardable = 1 << 0,   // can be resized to 0 at any time. Use for cached data
    Normal = 0 << 2,        // normal allocating alias. Don't use from ARM
    Direct = 1 << 2,        // 0xC alias uncached
    Coherent = 2 << 2,      // 0x8 alias. Non-allocating in L2 but coherent
    Zero = 1 << 4,          // initialise buffer to all zeros
    NoInit = 1 << 5,        // don't initialise (default is initialise to all ones)
    HintPermalock = 1 << 6, // Likely to be locked for long periods of time
    L1Nonallocating = (2 << 2 | 1 << 2), // Allocating in L2
}

pub(crate) struct VcAllocFlags(pub u32);

pub(crate) const DMA_MEM_FLAGS: VcAllocFlags =
    VcAllocFlags(VcAllocFlagsEnum::Direct as u32 | VcAllocFlagsEnum::Zero as u32);

pub(crate) const DMA_CHAN: usize = 10; // DMA channel to use
pub(crate) const DMA_ENABLE: usize = 0xff0;
pub(crate) const DMA_CS: usize = 0x00;
pub(crate) const DMA_CONBLK_AD: usize = 0x04;
pub(crate) const DMA_TI: usize = 0x08;
pub(crate) const DMA_SRCE_AD: usize = 0x0c;
pub(crate) const DMA_DEST_AD: usize = 0x10;
pub(crate) const DMA_TXFR_LEN: usize = 0x14;
pub(crate) const DMA_STRIDE: usize = 0x18;
pub(crate) const DMA_NEXTCONBK: usize = 0x1c;
pub(crate) const DMA_DEBUG: usize = 0x20;

pub(crate) const DMA_WAIT_RESP: u32 = 1 << 3;
pub(crate) const DMA_CB_DEST_INC: u32 = 1 << 4;
pub(crate) const DMA_DEST_DREQ: u32 = 1 << 6;
pub(crate) const DMA_CB_SRCE_INC: u32 = 1 << 8;
pub(crate) const DMA_SRCE_DREQ: u32 = 1 << 10;

pub(crate) const DMA_SMI_DREQ: u32 = 4;
macro_rules! DMA_PRIORITY {
    ($n:expr) => {
        (($n) << 16)
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct DmaCb {
    pub ti: u32,      // Transfer information
    pub srce_ad: u32, // Source address
    pub dest_ad: u32, // Destination address
    pub tfr_len: u32, // Transfer length
    pub stride: u32,  // Transfer stride
    pub next_cb: u32, // Next control block address
    pub debug: u32,   // Debug register, zero in control block
    _unusued: u32,
}

