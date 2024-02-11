use c2rust_bitfields::BitfieldStruct;
use libc::c_void;

use crate::consts::PHYS_REG_BASE;

pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;

pub(crate) const SMI_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x600000);
pub(crate) const SMI_CS: usize = 0x00; // Control & status
pub(crate) const SMI_L: usize = 0x04; // Transfer length
pub(crate) const SMI_A: usize = 0x08; // Address
pub(crate) const SMI_D: usize = 0x0c; // Data
pub(crate) const SMI_DSR0: usize = 0x10; // Read settings device 0
pub(crate) const SMI_DSW0: usize = 0x14; // Write settings device 0
pub(crate) const SMI_DSR1: usize = 0x18; // Read settings device 1
pub(crate) const SMI_DSW1: usize = 0x1c; // Write settings device 1
pub(crate) const SMI_DSR2: usize = 0x20; // Read settings device 2
pub(crate) const SMI_DSW2: usize = 0x24; // Write settings device 2
pub(crate) const SMI_DSR3: usize = 0x28; // Read settings device 3
pub(crate) const SMI_DSW3: usize = 0x2c; // Write settings device 3
pub(crate) const SMI_DMC: usize = 0x30; // DMA control
pub(crate) const SMI_DCS: usize = 0x34; // Direct control/status
pub(crate) const SMI_DCA: usize = 0x38; // Direct address
pub(crate) const SMI_DCD: usize = 0x3c; // Direct data
pub(crate) const SMI_FD: usize = 0x40; // FIFO debug

pub(crate) const CLK_SMI_CTL: usize = 0xb0;
pub(crate) const CLK_SMI_DIV: usize = 0xb4;
pub(crate) const CLK_PASSWD: usize = 0x5a000000;

#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_CS_REG {
    pub cs: SMI_CS_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_CS_REG_STRUCT {
    #[bitfield(name = "enable", ty = "uint32_t", bits = "0..=0")]
    #[bitfield(name = "done", ty = "uint32_t", bits = "1..=1")]
    #[bitfield(name = "active", ty = "uint32_t", bits = "2..=2")]
    #[bitfield(name = "start", ty = "uint32_t", bits = "3..=3")]
    #[bitfield(name = "clear", ty = "uint32_t", bits = "4..=4")]
    #[bitfield(name = "write", ty = "uint32_t", bits = "5..=5")]
    #[bitfield(name = "_x1", ty = "uint32_t", bits = "6..=7")]
    #[bitfield(name = "teen", ty = "uint32_t", bits = "8..=8")]
    #[bitfield(name = "intd", ty = "uint32_t", bits = "9..=9")]
    #[bitfield(name = "intt", ty = "uint32_t", bits = "10..=10")]
    #[bitfield(name = "intr", ty = "uint32_t", bits = "11..=11")]
    #[bitfield(name = "pvmode", ty = "uint32_t", bits = "12..=12")]
    #[bitfield(name = "seterr", ty = "uint32_t", bits = "13..=13")]
    #[bitfield(name = "pxldat", ty = "uint32_t", bits = "14..=14")]
    #[bitfield(name = "edreq", ty = "uint32_t", bits = "15..=15")]
    #[bitfield(name = "_x2", ty = "uint32_t", bits = "16..=23")]
    #[bitfield(name = "_x3", ty = "uint32_t", bits = "24..=24")]
    #[bitfield(name = "aferr", ty = "uint32_t", bits = "25..=25")]
    #[bitfield(name = "txw", ty = "uint32_t", bits = "26..=26")]
    #[bitfield(name = "rxr", ty = "uint32_t", bits = "27..=27")]
    #[bitfield(name = "txd", ty = "uint32_t", bits = "28..=28")]
    #[bitfield(name = "rxd", ty = "uint32_t", bits = "29..=29")]
    #[bitfield(name = "txe", ty = "uint32_t", bits = "30..=30")]
    #[bitfield(name = "rxf", ty = "uint32_t", bits = "31..=31")]
    pub enable_done_active_start_clear_write__x1_teen_intd_intt_intr_pvmode_seterr_pxldat_edreq__x2__x3_aferr_txw_rxr_txd_rxd_txe_rxf:
        [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_L_REG {
    pub l: SMI_L_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_L_REG_STRUCT {
    #[bitfield(name = "len", ty = "uint32_t", bits = "0..=31")]
    pub len: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_A_REG {
    pub a: SMI_A_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_A_REG_STRUCT {
    #[bitfield(name = "addr", ty = "uint32_t", bits = "0..=5")]
    #[bitfield(name = "_x1", ty = "uint32_t", bits = "6..=7")]
    #[bitfield(name = "dev", ty = "uint32_t", bits = "8..=9")]
    pub addr__x1_dev: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_D_REG {
    pub d: SMI_D_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_D_REG_STRUCT {
    #[bitfield(name = "data", ty = "uint32_t", bits = "0..=31")]
    pub data: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_DMC_REG {
    pub dmc: SMI_DMC_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_DMC_REG_STRUCT {
    #[bitfield(name = "reqw", ty = "uint32_t", bits = "0..=5")]
    #[bitfield(name = "reqr", ty = "uint32_t", bits = "6..=11")]
    #[bitfield(name = "panicw", ty = "uint32_t", bits = "12..=17")]
    #[bitfield(name = "panicr", ty = "uint32_t", bits = "18..=23")]
    #[bitfield(name = "dmap", ty = "uint32_t", bits = "24..=24")]
    #[bitfield(name = "_x1", ty = "uint32_t", bits = "25..=27")]
    #[bitfield(name = "dmaen", ty = "uint32_t", bits = "28..=28")]
    pub reqw_reqr_panicw_panicr_dmap__x1_dmaen: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_DSR_REG {
    pub dsr: SMI_DSR_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_DSR_REG_STRUCT {
    #[bitfield(name = "rstrobe", ty = "uint32_t", bits = "0..=6")]
    #[bitfield(name = "rdreq", ty = "uint32_t", bits = "7..=7")]
    #[bitfield(name = "rpace", ty = "uint32_t", bits = "8..=14")]
    #[bitfield(name = "rpaceall", ty = "uint32_t", bits = "15..=15")]
    #[bitfield(name = "rhold", ty = "uint32_t", bits = "16..=21")]
    #[bitfield(name = "fsetup", ty = "uint32_t", bits = "22..=22")]
    #[bitfield(name = "mode68", ty = "uint32_t", bits = "23..=23")]
    #[bitfield(name = "rsetup", ty = "uint32_t", bits = "24..=29")]
    #[bitfield(name = "rwidth", ty = "uint32_t", bits = "30..=31")]
    pub rstrobe_rdreq_rpace_rpaceall_rhold_fsetup_mode68_rsetup_rwidth: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_DSW_REG {
    pub dsw: SMI_DSW_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_DSW_REG_STRUCT {
    #[bitfield(name = "wstrobe", ty = "uint32_t", bits = "0..=6")]
    #[bitfield(name = "wdreq", ty = "uint32_t", bits = "7..=7")]
    #[bitfield(name = "wpace", ty = "uint32_t", bits = "8..=14")]
    #[bitfield(name = "wpaceall", ty = "uint32_t", bits = "15..=15")]
    #[bitfield(name = "whold", ty = "uint32_t", bits = "16..=21")]
    #[bitfield(name = "wswap", ty = "uint32_t", bits = "22..=22")]
    #[bitfield(name = "wformat", ty = "uint32_t", bits = "23..=23")]
    #[bitfield(name = "wsetup", ty = "uint32_t", bits = "24..=29")]
    #[bitfield(name = "wwidth", ty = "uint32_t", bits = "30..=31")]
    pub wstrobe_wdreq_wpace_wpaceall_whold_wswap_wformat_wsetup_wwidth: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_DCS_REG {
    pub dcs: SMI_DCS_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_DCS_REG_STRUCT {
    #[bitfield(name = "enable", ty = "uint32_t", bits = "0..=0")]
    #[bitfield(name = "start", ty = "uint32_t", bits = "1..=1")]
    #[bitfield(name = "done", ty = "uint32_t", bits = "2..=2")]
    #[bitfield(name = "write", ty = "uint32_t", bits = "3..=3")]
    pub enable_start_done_write: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_DCA_REG {
    pub dca: SMI_DCA_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_DCA_REG_STRUCT {
    #[bitfield(name = "addr", ty = "uint32_t", bits = "0..=5")]
    #[bitfield(name = "_x1", ty = "uint32_t", bits = "6..=7")]
    #[bitfield(name = "dev", ty = "uint32_t", bits = "8..=9")]
    pub addr__x1_dev: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_DCD_REG {
    pub dcd: SMI_DCD_REG_STRUCT,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SMI_DCD_REG_STRUCT {
    #[bitfield(name = "data", ty = "uint32_t", bits = "0..=31")]
    pub data: [u8; 4],
}

macro_rules! REG32 {
    ($name: ident, $offset: expr) => {
        (unsafe { $name.virt.wrapping_add($offset) }) as *mut u32
    };
}

macro_rules! set_bits {
    ($name: expr, $value: expr, $bits: expr, $offset: expr) => {
        $name
            .write_volatile($name.read_volatile() | ($value as u32 & ((1 << $bits) - 1)) << $offset)
    };
}

pub(crate) use {set_bits, REG32};

pub(crate) const SMI_8_BITS: usize = 0;
pub(crate) const SMI_16_BITS: usize = 1;
pub(crate) const SMI_18_BITS: usize = 2;
pub(crate) const SMI_9_BITS: usize = 3;
