#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::rpi_dma_utils::{
    clk_regs, dma_regs, enable_dma, gpio_mode, gpio_regs, map_periph, map_uncached_mem, start_dma,
    stop_dma, unmap_periph_mem, DMA_CB, MEM_MAP,
};
use c2rust_bitfields::BitfieldStruct;

extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __useconds_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_CS_REG {
    pub c2rust_unnamed: C2RustUnnamed,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed {
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
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "len", ty = "uint32_t", bits = "0..=31")]
    pub len: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_A_REG {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    #[bitfield(name = "data", ty = "uint32_t", bits = "0..=31")]
    pub data: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SMI_DMC_REG {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub value: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    #[bitfield(name = "data", ty = "uint32_t", bits = "0..=31")]
    pub data: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union color_t {
    pub packed: uint32_t,
    pub component: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub b: uint8_t,
    pub r: uint8_t,
    pub g: uint8_t,
    pub a: uint8_t,
}
#[no_mangle]
pub static mut vc_mem: MEM_MAP = MEM_MAP {
    fd: 0,
    h: 0,
    size: 0,
    bus: 0 as *const libc::c_void as *mut libc::c_void,
    virt: 0 as *const libc::c_void as *mut libc::c_void,
    phys: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
pub static mut smi_regs: MEM_MAP = MEM_MAP {
    fd: 0,
    h: 0,
    size: 0,
    bus: 0 as *const libc::c_void as *mut libc::c_void,
    virt: 0 as *const libc::c_void as *mut libc::c_void,
    phys: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
pub static mut smi_cs: *mut SMI_CS_REG = 0 as *const SMI_CS_REG as *mut SMI_CS_REG;
#[no_mangle]
pub static mut smi_l: *mut SMI_L_REG = 0 as *const SMI_L_REG as *mut SMI_L_REG;
#[no_mangle]
pub static mut smi_a: *mut SMI_A_REG = 0 as *const SMI_A_REG as *mut SMI_A_REG;
#[no_mangle]
pub static mut smi_d: *mut SMI_D_REG = 0 as *const SMI_D_REG as *mut SMI_D_REG;
#[no_mangle]
pub static mut smi_dmc: *mut SMI_DMC_REG = 0 as *const SMI_DMC_REG as *mut SMI_DMC_REG;
#[no_mangle]
pub static mut smi_dsr: *mut SMI_DSR_REG = 0 as *const SMI_DSR_REG as *mut SMI_DSR_REG;
#[no_mangle]
pub static mut smi_dsw: *mut SMI_DSW_REG = 0 as *const SMI_DSW_REG as *mut SMI_DSW_REG;
#[no_mangle]
pub static mut smi_dcs: *mut SMI_DCS_REG = 0 as *const SMI_DCS_REG as *mut SMI_DCS_REG;
#[no_mangle]
pub static mut smi_dca: *mut SMI_DCA_REG = 0 as *const SMI_DCA_REG as *mut SMI_DCA_REG;
#[no_mangle]
pub static mut smi_dcd: *mut SMI_DCD_REG = 0 as *const SMI_DCD_REG as *mut SMI_DCD_REG;
#[no_mangle]
pub static mut led_count: uint16_t = 0 as libc::c_int as uint16_t;
#[no_mangle]
pub static mut txdata: *mut uint16_t = 0 as *const uint16_t as *mut uint16_t;
#[no_mangle]
pub static mut tx_buffer: [uint16_t; 36872] = [0 as libc::c_int as uint16_t; 36872];
#[no_mangle]
pub static mut color_buffer: [[color_t; 512]; 16] = [[color_t { packed: 0 }; 512]; 16];
#[no_mangle]
pub unsafe extern "C" fn map_devices() {
    map_periph(
        &mut gpio_regs,
        (0xfe000000 as libc::c_uint).wrapping_add(0x200000 as libc::c_int as libc::c_uint)
            as *mut libc::c_void,
        0x1000 as libc::c_int,
    );
    map_periph(
        &mut dma_regs,
        (0xfe000000 as libc::c_uint).wrapping_add(0x7000 as libc::c_int as libc::c_uint)
            as *mut libc::c_void,
        0x1000 as libc::c_int,
    );
    map_periph(
        &mut clk_regs,
        (0xfe000000 as libc::c_uint).wrapping_add(0x101000 as libc::c_int as libc::c_uint)
            as *mut libc::c_void,
        0x1000 as libc::c_int,
    );
    map_periph(
        &mut smi_regs,
        (0xfe000000 as libc::c_uint).wrapping_add(0x600000 as libc::c_int as libc::c_uint)
            as *mut libc::c_void,
        0x1000 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fail(mut s: *mut libc::c_char) {
    printf(b"%s\0" as *const u8 as *const libc::c_char, s);
    terminate(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn terminate(mut _sig: libc::c_int) {
    let mut i: libc::c_int = 0;
    printf(b"Closing\n\0" as *const u8 as *const libc::c_char);
    if !(gpio_regs.virt).is_null() {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            gpio_mode(8 as libc::c_int + i, 0 as libc::c_int);
            i += 1;
            i;
        }
    }
    if !(smi_regs.virt).is_null() {
        ::core::ptr::write_volatile(
            (smi_regs.virt as uint32_t).wrapping_add(0 as libc::c_int as uint32_t) as *mut uint32_t,
            0 as libc::c_int as uint32_t,
        );
    }
    stop_dma(10 as libc::c_int);
    unmap_periph_mem(&mut vc_mem);
    unmap_periph_mem(&mut smi_regs);
    unmap_periph_mem(&mut dma_regs);
    unmap_periph_mem(&mut gpio_regs);
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn init_smi(
    mut width: libc::c_int,
    mut ns: libc::c_int,
    mut setup: libc::c_int,
    mut strobe: libc::c_int,
    mut hold: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut divi: libc::c_int = ns / 2 as libc::c_int;
    smi_cs = (smi_regs.virt as uint32_t).wrapping_add(0 as libc::c_int as uint32_t) as *mut uint32_t
        as *mut SMI_CS_REG as *mut SMI_CS_REG;
    smi_l = (smi_regs.virt as uint32_t).wrapping_add(0x4 as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_L_REG as *mut SMI_L_REG;
    smi_a = (smi_regs.virt as uint32_t).wrapping_add(0x8 as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_A_REG as *mut SMI_A_REG;
    smi_d = (smi_regs.virt as uint32_t).wrapping_add(0xc as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_D_REG as *mut SMI_D_REG;
    smi_dmc = (smi_regs.virt as uint32_t).wrapping_add(0x30 as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_DMC_REG as *mut SMI_DMC_REG;
    smi_dsr = (smi_regs.virt as uint32_t).wrapping_add(0x10 as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_DSR_REG as *mut SMI_DSR_REG;
    smi_dsw = (smi_regs.virt as uint32_t).wrapping_add(0x14 as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_DSW_REG as *mut SMI_DSW_REG;
    smi_dcs = (smi_regs.virt as uint32_t).wrapping_add(0x34 as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_DCS_REG as *mut SMI_DCS_REG;
    smi_dca = (smi_regs.virt as uint32_t).wrapping_add(0x38 as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_DCA_REG as *mut SMI_DCA_REG;
    smi_dcd = (smi_regs.virt as uint32_t).wrapping_add(0x3c as libc::c_int as uint32_t)
        as *mut uint32_t as *mut SMI_DCD_REG as *mut SMI_DCD_REG;
    ::core::ptr::write_volatile(
        &mut (*smi_a).value as *mut uint32_t,
        0 as libc::c_int as uint32_t,
    );
    ::core::ptr::write_volatile(
        &mut (*smi_l).value as *mut uint32_t,
        ::core::ptr::read_volatile::<uint32_t>(&(*smi_a).value as *const uint32_t),
    );
    ::core::ptr::write_volatile(
        &mut (*smi_cs).value as *mut uint32_t,
        ::core::ptr::read_volatile::<uint32_t>(&(*smi_l).value as *const uint32_t),
    );
    ::core::ptr::write_volatile(
        &mut (*smi_dca).value as *mut uint32_t,
        0 as libc::c_int as uint32_t,
    );
    ::core::ptr::write_volatile(
        &mut (*smi_dcs).value as *mut uint32_t,
        ::core::ptr::read_volatile::<uint32_t>(&(*smi_dca).value as *const uint32_t),
    );
    ::core::ptr::write_volatile(
        &mut (*smi_dsw).value as *mut uint32_t,
        ::core::ptr::read_volatile::<uint32_t>(&(*smi_dcs).value as *const uint32_t),
    );
    ::core::ptr::write_volatile(
        &mut (*smi_dsr).value as *mut uint32_t,
        ::core::ptr::read_volatile::<uint32_t>(&(*smi_dsw).value as *const uint32_t),
    );
    if *((clk_regs.virt as uint32_t).wrapping_add(0xb4 as libc::c_int as uint32_t) as *mut uint32_t)
        != (divi << 12 as libc::c_int) as uint32_t
    {
        ::core::ptr::write_volatile(
            (clk_regs.virt as uint32_t).wrapping_add(0xb0 as libc::c_int as uint32_t)
                as *mut uint32_t,
            (0x5a000000 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int) as uint32_t,
        );
        usleep(10 as libc::c_int as __useconds_t);
        while *((clk_regs.virt as uint32_t).wrapping_add(0xb0 as libc::c_int as uint32_t)
            as *mut uint32_t)
            & ((1 as libc::c_int) << 7 as libc::c_int) as uint32_t
            != 0
        {}
        usleep(10 as libc::c_int as __useconds_t);
        ::core::ptr::write_volatile(
            (clk_regs.virt as uint32_t).wrapping_add(0xb4 as libc::c_int as uint32_t)
                as *mut uint32_t,
            (0x5a000000 as libc::c_int | divi << 12 as libc::c_int) as uint32_t,
        );
        usleep(10 as libc::c_int as __useconds_t);
        ::core::ptr::write_volatile(
            (clk_regs.virt as uint32_t).wrapping_add(0xb0 as libc::c_int as uint32_t)
                as *mut uint32_t,
            (0x5a000000 as libc::c_int | 6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as uint32_t,
        );
        usleep(10 as libc::c_int as __useconds_t);
        while *((clk_regs.virt as uint32_t).wrapping_add(0xb0 as libc::c_int as uint32_t)
            as *mut uint32_t)
            & ((1 as libc::c_int) << 7 as libc::c_int) as uint32_t
            == 0 as libc::c_int as uint32_t
        {}
        usleep(100 as libc::c_int as __useconds_t);
    }
    if ((*smi_cs).c2rust_unnamed).seterr() != 0 {
        ((*smi_cs).c2rust_unnamed).set_seterr(1 as libc::c_int as uint32_t);
    }
    ((*smi_dsw).c2rust_unnamed).set_wsetup(setup as uint32_t);
    ((*smi_dsr).c2rust_unnamed).set_rsetup(((*smi_dsw).c2rust_unnamed).wsetup());
    ((*smi_dsw).c2rust_unnamed).set_wstrobe(strobe as uint32_t);
    ((*smi_dsr).c2rust_unnamed).set_rstrobe(((*smi_dsw).c2rust_unnamed).wstrobe());
    ((*smi_dsw).c2rust_unnamed).set_whold(hold as uint32_t);
    ((*smi_dsr).c2rust_unnamed).set_rhold(((*smi_dsw).c2rust_unnamed).whold());
    ((*smi_dmc).c2rust_unnamed).set_panicw(8 as libc::c_int as uint32_t);
    ((*smi_dmc).c2rust_unnamed).set_panicr(((*smi_dmc).c2rust_unnamed).panicw());
    ((*smi_dmc).c2rust_unnamed).set_reqw(2 as libc::c_int as uint32_t);
    ((*smi_dmc).c2rust_unnamed).set_reqr(((*smi_dmc).c2rust_unnamed).reqw());
    ((*smi_dsw).c2rust_unnamed).set_wwidth(width as uint32_t);
    ((*smi_dsr).c2rust_unnamed).set_rwidth(((*smi_dsw).c2rust_unnamed).wwidth());
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        gpio_mode(8 as libc::c_int + i, 5 as libc::c_int);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn setup_smi_dma(mut mp: *mut MEM_MAP, mut nsamp: libc::c_int) {
    let mut cbs: *mut DMA_CB = (*mp).virt as *mut DMA_CB;
    txdata = cbs.offset(1 as libc::c_int as isize) as *mut uint16_t;
    ((*smi_dmc).c2rust_unnamed).set_dmaen(1 as libc::c_int as uint32_t);
    ((*smi_cs).c2rust_unnamed).set_enable(1 as libc::c_int as uint32_t);
    ((*smi_cs).c2rust_unnamed).set_clear(1 as libc::c_int as uint32_t);
    ((*smi_cs).c2rust_unnamed).set_pxldat(1 as libc::c_int as uint32_t);
    ((*smi_l).c2rust_unnamed).set_len(
        (nsamp as libc::c_ulong).wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong)
            as uint32_t,
    );
    ((*smi_cs).c2rust_unnamed).set_write(1 as libc::c_int as uint32_t);
    enable_dma(10 as libc::c_int);
    (*cbs.offset(0 as libc::c_int as isize)).ti = ((1 as libc::c_int) << 6 as libc::c_int
        | (4 as libc::c_int) << 16 as libc::c_int
        | (1 as libc::c_int) << 8 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int)
        as uint32_t;
    (*cbs.offset(0 as libc::c_int as isize)).tfr_len = (nsamp as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong)
        as uint32_t;
    (*cbs.offset(0 as libc::c_int as isize)).srce_ad = (txdata as uint32_t)
        .wrapping_sub((*mp).virt as uint32_t)
        .wrapping_add((*mp).bus as uint32_t);
    (*cbs.offset(0 as libc::c_int as isize)).dest_ad =
        (smi_regs.bus as uint32_t).wrapping_add(0xc as libc::c_int as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn start_smi(mut mp: *mut MEM_MAP) {
    let mut cbs: *mut DMA_CB = (*mp).virt as *mut DMA_CB;
    start_dma(
        mp,
        10 as libc::c_int,
        &mut *cbs.offset(0 as libc::c_int as isize),
        0 as libc::c_int as uint32_t,
    );
    ((*smi_cs).c2rust_unnamed).set_start(1 as libc::c_int as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn swap_bytes(mut data: *mut libc::c_void, mut len: libc::c_int) {
    let mut wp: *mut uint16_t = data as *mut uint16_t;
    len = (len + 1 as libc::c_int) / 2 as libc::c_int;
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        *wp = (*wp).swap_bytes();
        wp = wp.offset(1);
        wp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn leds_init(mut init_led_count: libc::c_int) -> libc::c_int {
    if init_led_count > 512 as libc::c_int {
        printf(
            b"smileds: Error! Max %d leds supported!\n\0" as *const u8 as *const libc::c_char,
            512 as libc::c_int,
        );
    }
    led_count = init_led_count as uint16_t;
    map_devices();
    init_smi(
        if 16 as libc::c_int > 8 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        },
        10 as libc::c_int,
        15 as libc::c_int,
        30 as libc::c_int,
        15 as libc::c_int,
    );
    map_uncached_mem(
        &mut vc_mem,
        (0x1000 as libc::c_int as libc::c_ulong).wrapping_add(
            ((4 as libc::c_int
                + 24 as libc::c_int * 3 as libc::c_int * 512 as libc::c_int
                + 4 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as libc::c_int,
    );
    setup_smi_dma(
        &mut vc_mem,
        4 as libc::c_int
            + 24 as libc::c_int * 3 as libc::c_int * led_count as libc::c_int
            + 4 as libc::c_int,
    );
    let mut tx_offset: *mut uint16_t = &mut *tx_buffer.as_mut_ptr().offset(
        (4 as libc::c_int + 24 as libc::c_int * 3 as libc::c_int * 0 as libc::c_int) as isize,
    ) as *mut uint16_t;
    let mut b: uint32_t = 0 as libc::c_int as uint32_t;
    while b < (led_count as libc::c_int * 24 as libc::c_int) as uint32_t {
        *tx_offset.offset(0 as libc::c_int as isize) = 0xffff as libc::c_int as uint16_t;
        *tx_offset.offset(1 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
        *tx_offset.offset(2 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
        tx_offset = tx_offset.offset(3 as libc::c_int as isize);
        b = b.wrapping_add(1);
        b;
    }
    leds_clear();
    printf(
        b"smileds: Setting %u LED%s per channel, %u channels\n\0" as *const u8
            as *const libc::c_char,
        led_count as libc::c_int,
        if led_count as libc::c_int == 1 as libc::c_int {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"s\0" as *const u8 as *const libc::c_char
        },
        16 as libc::c_int,
    );
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn leds_get_buffer() -> *mut uint16_t {
    return tx_buffer.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn leds_set_pixel(
    mut channel: uint8_t,
    mut pixel: uint16_t,
    mut color: color_t,
) {
    if pixel as libc::c_int >= led_count as libc::c_int {
        return;
    }
    if channel as libc::c_int > 16 as libc::c_int {
        return;
    }
    if color.component.a as libc::c_int == 255 as libc::c_int {
        color_buffer[channel as usize][pixel as usize] = color;
    } else {
        let old_a: uint8_t = (1 as libc::c_int - color.component.a as libc::c_int) as uint8_t;
        color.component.r = (color_buffer[channel as usize][pixel as usize].component.r
            as libc::c_int
            * old_a as libc::c_int
            / 255 as libc::c_int
            + color.component.r as libc::c_int * color.component.a as libc::c_int
                / 255 as libc::c_int) as uint8_t;
        color.component.g = (color_buffer[channel as usize][pixel as usize].component.g
            as libc::c_int
            * old_a as libc::c_int
            / 255 as libc::c_int
            + color.component.g as libc::c_int * color.component.a as libc::c_int
                / 255 as libc::c_int) as uint8_t;
        color.component.b = (color_buffer[channel as usize][pixel as usize].component.b
            as libc::c_int
            * old_a as libc::c_int
            / 255 as libc::c_int
            + color.component.b as libc::c_int * color.component.a as libc::c_int
                / 255 as libc::c_int) as uint8_t;
        color_buffer[channel as usize][pixel as usize] = color;
    }
    let channel_on_mask: uint16_t = ((1 as libc::c_int) << channel as libc::c_int) as uint16_t;
    let channel_off_mask: uint16_t = !((1 as libc::c_int) << channel as libc::c_int) as uint16_t;
    let mut rgb_mask: uint32_t = ((1 as libc::c_int) << 23 as libc::c_int) as uint32_t;
    let mut tx_offset: *mut uint16_t = &mut *tx_buffer.as_mut_ptr().offset(
        (4 as libc::c_int + 24 as libc::c_int * 3 as libc::c_int * pixel as libc::c_int) as isize,
    ) as *mut uint16_t;
    let mut n: uint8_t = 0 as libc::c_int as uint8_t;
    while (n as libc::c_int) < 24 as libc::c_int {
        if color.packed & rgb_mask != 0 {
            let ref mut fresh1 = *tx_offset.offset(1 as libc::c_int as isize);
            *fresh1 = (*fresh1 as libc::c_int | channel_on_mask as libc::c_int) as uint16_t;
        } else {
            let ref mut fresh2 = *tx_offset.offset(1 as libc::c_int as isize);
            *fresh2 = (*fresh2 as libc::c_int & channel_off_mask as libc::c_int) as uint16_t;
        }
        tx_offset = tx_offset.offset(3 as libc::c_int as isize);
        rgb_mask = rgb_mask >> 1 as libc::c_int;
        n = n.wrapping_add(1);
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn leds_clear() {
    let mut tx_offset: *mut uint16_t = &mut *tx_buffer.as_mut_ptr().offset(
        (4 as libc::c_int + 24 as libc::c_int * 3 as libc::c_int * 0 as libc::c_int) as isize,
    ) as *mut uint16_t;
    let mut b: uint32_t = 0 as libc::c_int as uint32_t;
    while b < (led_count as libc::c_int * 24 as libc::c_int) as uint32_t {
        *tx_offset.offset(1 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
        tx_offset = tx_offset.offset(3 as libc::c_int as isize);
        b = b.wrapping_add(1);
        b;
    }
    memset(
        color_buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[[color_t; 512]; 16]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn leds_send() {
    memcpy(
        txdata as *mut libc::c_void,
        tx_buffer.as_mut_ptr() as *const libc::c_void,
        ((4 as libc::c_int
            + 24 as libc::c_int * 3 as libc::c_int * led_count as libc::c_int
            + 4 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
    );
    start_smi(&mut vc_mem);
}
