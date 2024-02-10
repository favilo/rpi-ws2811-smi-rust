#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn close(_: libc::c_int) -> libc::c_int;
    fn usleep(_: libc::c_uint) -> libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn ioctl(_: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    fn mmap(
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: off_t,
    ) -> *mut libc::c_void;
    fn munmap(_: *mut libc::c_void, _: size_t) -> libc::c_int;
    fn fail(s: *mut libc::c_char);
}
pub type uint8_t = libc::c_uchar;
pub type uint32_t = libc::c_uint;
pub type size_t = libc::c_uint;
pub type off_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEM_MAP {
    pub fd: libc::c_int,
    pub h: libc::c_int,
    pub size: libc::c_int,
    pub bus: *mut libc::c_void,
    pub virt: *mut libc::c_void,
    pub phys: *mut libc::c_void,
}
pub type VC_ALLOC_FLAGS = libc::c_uint;
pub const MEM_FLAG_L1_NONALLOCATING: VC_ALLOC_FLAGS = 12;
pub const MEM_FLAG_HINT_PERMALOCK: VC_ALLOC_FLAGS = 64;
pub const MEM_FLAG_NO_INIT: VC_ALLOC_FLAGS = 32;
pub const MEM_FLAG_ZERO: VC_ALLOC_FLAGS = 16;
pub const MEM_FLAG_COHERENT: VC_ALLOC_FLAGS = 8;
pub const MEM_FLAG_DIRECT: VC_ALLOC_FLAGS = 4;
pub const MEM_FLAG_NORMAL: VC_ALLOC_FLAGS = 0;
pub const MEM_FLAG_DISCARDABLE: VC_ALLOC_FLAGS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VC_MSG {
    pub len: uint32_t,
    pub req: uint32_t,
    pub tag: uint32_t,
    pub blen: uint32_t,
    pub dlen: uint32_t,
    pub uints: [uint32_t; 27],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DMA_CB {
    pub ti: uint32_t,
    pub srce_ad: uint32_t,
    pub dest_ad: uint32_t,
    pub tfr_len: uint32_t,
    pub stride: uint32_t,
    pub next_cb: uint32_t,
    pub debug: uint32_t,
    pub unused: uint32_t,
}
#[no_mangle]
pub static mut dma_regstrs: [*mut libc::c_char; 10] = [
    b"DMA CS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CB_AD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"TI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SRCE_AD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"DEST_AD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"TFR_LEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"STRIDE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"NEXT_CB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"DEBUG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut gpio_mode_strs: [*mut libc::c_char; 8] = [
    b"IN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OUT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALT5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALT4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALT0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALT1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALT2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALT3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut pwm_regs: MEM_MAP = MEM_MAP {
    fd: 0,
    h: 0,
    size: 0,
    bus: 0 as *const libc::c_void as *mut libc::c_void,
    virt: 0 as *const libc::c_void as *mut libc::c_void,
    phys: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
pub static mut gpio_regs: MEM_MAP = MEM_MAP {
    fd: 0,
    h: 0,
    size: 0,
    bus: 0 as *const libc::c_void as *mut libc::c_void,
    virt: 0 as *const libc::c_void as *mut libc::c_void,
    phys: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
pub static mut dma_regs: MEM_MAP = MEM_MAP {
    fd: 0,
    h: 0,
    size: 0,
    bus: 0 as *const libc::c_void as *mut libc::c_void,
    virt: 0 as *const libc::c_void as *mut libc::c_void,
    phys: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
pub static mut clk_regs: MEM_MAP = MEM_MAP {
    fd: 0,
    h: 0,
    size: 0,
    bus: 0 as *const libc::c_void as *mut libc::c_void,
    virt: 0 as *const libc::c_void as *mut libc::c_void,
    phys: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
pub unsafe extern "C" fn map_periph(
    mut mp: *mut MEM_MAP,
    mut phys: *mut libc::c_void,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    (*mp).phys = phys;
    (*mp)
        .size = if size % 0x1000 as libc::c_int == 0 as libc::c_int {
        size
    } else {
        size + 0x1000 as libc::c_int & !(0x1000 as libc::c_int - 1 as libc::c_int)
    };
    (*mp)
        .bus = (phys as uint32_t)
        .wrapping_sub(0xfe000000 as libc::c_uint)
        .wrapping_add(0x7e000000 as libc::c_int as libc::c_uint) as *mut libc::c_void;
    (*mp).virt = map_segment(phys, (*mp).size);
    return (*mp).virt;
}
#[no_mangle]
pub unsafe extern "C" fn map_uncached_mem(
    mut mp: *mut MEM_MAP,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    (*mp)
        .size = if size % 0x1000 as libc::c_int == 0 as libc::c_int {
        size
    } else {
        size + 0x1000 as libc::c_int & !(0x1000 as libc::c_int - 1 as libc::c_int)
    };
    (*mp).fd = open_mbox();
    (*mp)
        .h = alloc_vc_mem(
        (*mp).fd,
        (*mp).size as uint32_t,
        (MEM_FLAG_DIRECT as libc::c_int | MEM_FLAG_ZERO as libc::c_int) as VC_ALLOC_FLAGS,
    ) as libc::c_int;
    ret = if (*mp).h > 0 as libc::c_int
        && {
            (*mp).bus = lock_vc_mem((*mp).fd, (*mp).h);
            !((*mp).bus).is_null()
        }
        && {
            (*mp)
                .virt = map_segment(
                ((*mp).bus as uint32_t & !(0xc0000000 as libc::c_uint))
                    as *mut libc::c_void,
                (*mp).size,
            );
            !((*mp).virt).is_null()
        }
    {
        (*mp).virt
    } else {
        0 as *mut libc::c_void
    };
    printf(
        b"VC mem handle %u, phys %p, virt %p\n\0" as *const u8 as *const libc::c_char,
        (*mp).h,
        (*mp).bus,
        (*mp).virt,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn unmap_periph_mem(mut mp: *mut MEM_MAP) {
    if !mp.is_null() {
        if (*mp).fd != 0 {
            unmap_segment((*mp).virt, (*mp).size);
            unlock_vc_mem((*mp).fd, (*mp).h);
            free_vc_mem((*mp).fd, (*mp).h);
            close_mbox((*mp).fd);
        } else {
            unmap_segment((*mp).virt, (*mp).size);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gpio_set(
    mut pin: libc::c_int,
    mut mode: libc::c_int,
    mut pull: libc::c_int,
) {
    gpio_mode(pin, mode);
    gpio_pull(pin, pull);
}
#[no_mangle]
pub unsafe extern "C" fn gpio_pull(mut pin: libc::c_int, mut pull: libc::c_int) {
    let mut reg: *mut uint32_t = ((gpio_regs.virt as uint32_t)
        .wrapping_add(0x98 as libc::c_int as uint32_t) as *mut uint32_t)
        .offset((pin / 32 as libc::c_int) as isize);
    ::core::ptr::write_volatile(
        (gpio_regs.virt as uint32_t).wrapping_add(0x94 as libc::c_int as uint32_t)
            as *mut uint32_t,
        pull as uint32_t,
    );
    usleep(2 as libc::c_int as libc::c_uint);
    ::core::ptr::write_volatile(
        reg,
        ((1 as libc::c_int) << pin % 32 as libc::c_int) as uint32_t,
    );
    usleep(2 as libc::c_int as libc::c_uint);
    ::core::ptr::write_volatile(
        (gpio_regs.virt as uint32_t).wrapping_add(0x94 as libc::c_int as uint32_t)
            as *mut uint32_t,
        0 as libc::c_int as uint32_t,
    );
    ::core::ptr::write_volatile(reg, 0 as libc::c_int as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn gpio_mode(mut pin: libc::c_int, mut mode: libc::c_int) {
    let mut reg: *mut uint32_t = ((gpio_regs.virt as uint32_t)
        .wrapping_add(0 as libc::c_int as uint32_t) as *mut uint32_t)
        .offset((pin / 10 as libc::c_int) as isize);
    let mut shift: uint32_t = (pin % 10 as libc::c_int * 3 as libc::c_int) as uint32_t;
    ::core::ptr::write_volatile(
        reg,
        *reg & !((7 as libc::c_int) << shift) as uint32_t | (mode << shift) as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gpio_out(mut pin: libc::c_int, mut val: libc::c_int) {
    let mut reg: *mut uint32_t = ((gpio_regs.virt as uint32_t)
        .wrapping_add(
            (if val != 0 { 0x1c as libc::c_int } else { 0x28 as libc::c_int })
                as uint32_t,
        ) as *mut uint32_t)
        .offset((pin / 32 as libc::c_int) as isize);
    ::core::ptr::write_volatile(
        reg,
        ((1 as libc::c_int) << pin % 32 as libc::c_int) as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gpio_in(mut pin: libc::c_int) -> uint8_t {
    let mut reg: *mut uint32_t = ((gpio_regs.virt as uint32_t)
        .wrapping_add(0x34 as libc::c_int as uint32_t) as *mut uint32_t)
        .offset((pin / 32 as libc::c_int) as isize);
    return (*reg >> pin % 32 as libc::c_int & 1 as libc::c_int as uint32_t) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn disp_mode_vals(mut mode: uint32_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        printf(
            b"%u:%-4s \0" as *const u8 as *const libc::c_char,
            i,
            gpio_mode_strs[(mode >> i * 3 as libc::c_int & 7 as libc::c_int as uint32_t)
                as usize],
        );
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn open_mbox() -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = open(b"/dev/vcio\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        fail(
            b"Error: can't open VC mailbox\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn close_mbox(mut fd: libc::c_int) {
    if fd >= 0 as libc::c_int {
        close(fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn msg_mbox(
    mut fd: libc::c_int,
    mut msgp: *mut VC_MSG,
) -> uint32_t {
    let mut ret: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0;
    i = (*msgp).dlen / 4 as libc::c_int as uint32_t;
    while i <= (*msgp).blen / 4 as libc::c_int as uint32_t {
        let fresh0 = i;
        i = i.wrapping_add(1);
        (*msgp).uints[fresh0 as usize] = 0 as libc::c_int as uint32_t;
        i = i.wrapping_add(4 as libc::c_int as uint32_t);
    }
    (*msgp)
        .len = ((*msgp).blen).wrapping_add(6 as libc::c_int as uint32_t)
        * 4 as libc::c_int as uint32_t;
    (*msgp).req = 0 as libc::c_int as uint32_t;
    if ioctl(
        fd,
        (((2 as libc::c_uint | 1 as libc::c_uint) << 30 as libc::c_int
            | ((100 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
            | 0 as libc::c_int as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                << 16 as libc::c_int) as libc::c_int,
        msgp,
    ) < 0 as libc::c_int
    {
        printf(b"VC IOCTL failed\n\0" as *const u8 as *const libc::c_char);
    } else if (*msgp).req & 0x80000000 as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        printf(b"VC IOCTL error\n\0" as *const u8 as *const libc::c_char);
    } else if (*msgp).req == 0x80000001 as libc::c_uint {
        printf(b"VC IOCTL partial error\n\0" as *const u8 as *const libc::c_char);
    } else {
        ret = (*msgp).uints[0 as libc::c_int as usize];
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_vc_mem(
    mut fd: libc::c_int,
    mut size: uint32_t,
    mut flags: VC_ALLOC_FLAGS,
) -> uint32_t {
    let mut msg: VC_MSG = {
        let mut init = VC_MSG {
            len: 0,
            req: 0,
            tag: 0x3000c as libc::c_int as uint32_t,
            blen: 12 as libc::c_int as uint32_t,
            dlen: 12 as libc::c_int as uint32_t,
            uints: [
                if size % 0x1000 as libc::c_int as uint32_t
                    == 0 as libc::c_int as uint32_t
                {
                    size
                } else {
                    size.wrapping_add(0x1000 as libc::c_int as uint32_t)
                        & !(0x1000 as libc::c_int - 1 as libc::c_int) as uint32_t
                },
                0x1000 as libc::c_int as uint32_t,
                flags as uint32_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    };
    return msg_mbox(fd, &mut msg);
}
#[no_mangle]
pub unsafe extern "C" fn lock_vc_mem(
    mut fd: libc::c_int,
    mut h: libc::c_int,
) -> *mut libc::c_void {
    let mut msg: VC_MSG = {
        let mut init = VC_MSG {
            len: 0,
            req: 0,
            tag: 0x3000d as libc::c_int as uint32_t,
            blen: 4 as libc::c_int as uint32_t,
            dlen: 4 as libc::c_int as uint32_t,
            uints: [
                h as uint32_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    };
    return if h != 0 {
        msg_mbox(fd, &mut msg) as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn unlock_vc_mem(
    mut fd: libc::c_int,
    mut h: libc::c_int,
) -> uint32_t {
    let mut msg: VC_MSG = {
        let mut init = VC_MSG {
            len: 0,
            req: 0,
            tag: 0x3000e as libc::c_int as uint32_t,
            blen: 4 as libc::c_int as uint32_t,
            dlen: 4 as libc::c_int as uint32_t,
            uints: [
                h as uint32_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    };
    return if h != 0 { msg_mbox(fd, &mut msg) } else { 0 as libc::c_int as uint32_t };
}
#[no_mangle]
pub unsafe extern "C" fn free_vc_mem(
    mut fd: libc::c_int,
    mut h: libc::c_int,
) -> uint32_t {
    let mut msg: VC_MSG = {
        let mut init = VC_MSG {
            len: 0,
            req: 0,
            tag: 0x3000f as libc::c_int as uint32_t,
            blen: 4 as libc::c_int as uint32_t,
            dlen: 4 as libc::c_int as uint32_t,
            uints: [
                h as uint32_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    };
    return if h != 0 { msg_mbox(fd, &mut msg) } else { 0 as libc::c_int as uint32_t };
}
#[no_mangle]
pub unsafe extern "C" fn set_vc_clock(
    mut fd: libc::c_int,
    mut id: libc::c_int,
    mut freq: uint32_t,
) -> uint32_t {
    let mut msg1: VC_MSG = {
        let mut init = VC_MSG {
            len: 0,
            req: 0,
            tag: 0x38001 as libc::c_int as uint32_t,
            blen: 8 as libc::c_int as uint32_t,
            dlen: 8 as libc::c_int as uint32_t,
            uints: [
                id as uint32_t,
                1 as libc::c_int as uint32_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    };
    let mut msg2: VC_MSG = {
        let mut init = VC_MSG {
            len: 0,
            req: 0,
            tag: 0x38002 as libc::c_int as uint32_t,
            blen: 12 as libc::c_int as uint32_t,
            dlen: 12 as libc::c_int as uint32_t,
            uints: [
                id as uint32_t,
                freq,
                0 as libc::c_int as uint32_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    };
    msg_mbox(fd, &mut msg1);
    disp_vc_msg(&mut msg1);
    msg_mbox(fd, &mut msg2);
    disp_vc_msg(&mut msg2);
    return 0 as libc::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn disp_vc_msg(mut msgp: *mut VC_MSG) {
    let mut i: libc::c_int = 0;
    printf(
        b"VC msg len=%X, req=%X, tag=%X, blen=%x, dlen=%x, data \0" as *const u8
            as *const libc::c_char,
        (*msgp).len,
        (*msgp).req,
        (*msgp).tag,
        (*msgp).blen,
        (*msgp).dlen,
    );
    i = 0 as libc::c_int;
    while (i as uint32_t) < (*msgp).blen / 4 as libc::c_int as uint32_t {
        printf(
            b"%08X \0" as *const u8 as *const libc::c_char,
            (*msgp).uints[i as usize],
        );
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn map_segment(
    mut addr: *mut libc::c_void,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    let mut fd: libc::c_int = 0;
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    size = if size % 0x1000 as libc::c_int == 0 as libc::c_int {
        size
    } else {
        size + 0x1000 as libc::c_int & !(0x1000 as libc::c_int - 1 as libc::c_int)
    };
    fd = open(
        b"/dev/mem\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int | 0o4010000 as libc::c_int | 0o2000000 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        fail(
            b"Error: can't open /dev/mem, run using sudo\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    mem = mmap(
        0 as *mut libc::c_void,
        size as size_t,
        2 as libc::c_int | 1 as libc::c_int,
        0x1 as libc::c_int,
        fd,
        addr as uint32_t as off_t,
    );
    close(fd);
    if mem == -(1 as libc::c_int) as *mut libc::c_void {
        fail(
            b"Error: can't map memory\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return mem;
}
#[no_mangle]
pub unsafe extern "C" fn unmap_segment(
    mut mem: *mut libc::c_void,
    mut size: libc::c_int,
) {
    if !mem.is_null() {
        munmap(
            mem,
            (if size % 0x1000 as libc::c_int == 0 as libc::c_int {
                size
            } else {
                size + 0x1000 as libc::c_int
                    & !(0x1000 as libc::c_int - 1 as libc::c_int)
            }) as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn enable_dma(mut chan: libc::c_int) {
    let ref mut fresh1 = *((dma_regs.virt as uint32_t)
        .wrapping_add(0xff0 as libc::c_int as uint32_t) as *mut uint32_t);
    ::core::ptr::write_volatile(
        fresh1,
        ::core::ptr::read_volatile::<uint32_t>(fresh1 as *const uint32_t)
            | ((1 as libc::c_int) << chan) as uint32_t,
    );
    ::core::ptr::write_volatile(
        (dma_regs.virt as uint32_t)
            .wrapping_add(
                (if 0 as libc::c_int == 0xff0 as libc::c_int {
                    0xff0 as libc::c_int
                } else {
                    chan * 0x100 as libc::c_int + 0 as libc::c_int
                }) as uint32_t,
            ) as *mut uint32_t,
        ((1 as libc::c_int) << 31 as libc::c_int) as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn start_dma(
    mut mp: *mut MEM_MAP,
    mut chan: libc::c_int,
    mut cbp: *mut DMA_CB,
    mut csval: uint32_t,
) {
    ::core::ptr::write_volatile(
        (dma_regs.virt as uint32_t)
            .wrapping_add(
                (if 0x4 as libc::c_int == 0xff0 as libc::c_int {
                    0xff0 as libc::c_int
                } else {
                    chan * 0x100 as libc::c_int + 0x4 as libc::c_int
                }) as uint32_t,
            ) as *mut uint32_t,
        (cbp as uint32_t)
            .wrapping_sub((*mp).virt as uint32_t)
            .wrapping_add((*mp).bus as uint32_t),
    );
    ::core::ptr::write_volatile(
        (dma_regs.virt as uint32_t)
            .wrapping_add(
                (if 0 as libc::c_int == 0xff0 as libc::c_int {
                    0xff0 as libc::c_int
                } else {
                    chan * 0x100 as libc::c_int + 0 as libc::c_int
                }) as uint32_t,
            ) as *mut uint32_t,
        2 as libc::c_int as uint32_t,
    );
    ::core::ptr::write_volatile(
        (dma_regs.virt as uint32_t)
            .wrapping_add(
                (if 0x20 as libc::c_int == 0xff0 as libc::c_int {
                    0xff0 as libc::c_int
                } else {
                    chan * 0x100 as libc::c_int + 0x20 as libc::c_int
                }) as uint32_t,
            ) as *mut uint32_t,
        7 as libc::c_int as uint32_t,
    );
    ::core::ptr::write_volatile(
        (dma_regs.virt as uint32_t)
            .wrapping_add(
                (if 0 as libc::c_int == 0xff0 as libc::c_int {
                    0xff0 as libc::c_int
                } else {
                    chan * 0x100 as libc::c_int + 0 as libc::c_int
                }) as uint32_t,
            ) as *mut uint32_t,
        1 as libc::c_int as uint32_t | csval,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dma_transfer_len(mut chan: libc::c_int) -> uint32_t {
    return *((dma_regs.virt as uint32_t)
        .wrapping_add(
            (if 0x14 as libc::c_int == 0xff0 as libc::c_int {
                0xff0 as libc::c_int
            } else {
                chan * 0x100 as libc::c_int + 0x14 as libc::c_int
            }) as uint32_t,
        ) as *mut uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn dma_active(mut chan: libc::c_int) -> uint32_t {
    return *((dma_regs.virt as uint32_t)
        .wrapping_add(
            (if 0 as libc::c_int == 0xff0 as libc::c_int {
                0xff0 as libc::c_int
            } else {
                chan * 0x100 as libc::c_int + 0 as libc::c_int
            }) as uint32_t,
        ) as *mut uint32_t) & 1 as libc::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn stop_dma(mut chan: libc::c_int) {
    if !(dma_regs.virt).is_null() {
        ::core::ptr::write_volatile(
            (dma_regs.virt as uint32_t)
                .wrapping_add(
                    (if 0 as libc::c_int == 0xff0 as libc::c_int {
                        0xff0 as libc::c_int
                    } else {
                        chan * 0x100 as libc::c_int + 0 as libc::c_int
                    }) as uint32_t,
                ) as *mut uint32_t,
            ((1 as libc::c_int) << 31 as libc::c_int) as uint32_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn disp_dma(mut chan: libc::c_int) {
    let mut p: *mut uint32_t = (dma_regs.virt as uint32_t)
        .wrapping_add(
            (if 0 as libc::c_int == 0xff0 as libc::c_int {
                0xff0 as libc::c_int
            } else {
                chan * 0x100 as libc::c_int + 0 as libc::c_int
            }) as uint32_t,
        ) as *mut uint32_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while *(dma_regstrs[i as usize]).offset(0 as libc::c_int as isize) != 0 {
        let fresh2 = i;
        i = i + 1;
        let fresh3 = p;
        p = p.offset(1);
        printf(
            b"%-7s %08X \0" as *const u8 as *const libc::c_char,
            dma_regstrs[fresh2 as usize],
            *fresh3,
        );
        if i % 5 as libc::c_int == 0 as libc::c_int
            || *(dma_regstrs[i as usize]).offset(0 as libc::c_int as isize)
                as libc::c_int == 0 as libc::c_int
        {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_pwm(
    mut freq: libc::c_int,
    mut range: libc::c_int,
    mut val: libc::c_int,
) {
    stop_pwm();
    if *((pwm_regs.virt as uint32_t).wrapping_add(0x4 as libc::c_int as uint32_t)
        as *mut uint32_t) & 0x100 as libc::c_int as uint32_t != 0
    {
        printf(b"PWM bus error\n\0" as *const u8 as *const libc::c_char);
        ::core::ptr::write_volatile(
            (pwm_regs.virt as uint32_t).wrapping_add(0x4 as libc::c_int as uint32_t)
                as *mut uint32_t,
            0x100 as libc::c_int as uint32_t,
        );
    }
    let mut divi: libc::c_int = 250000000 as libc::c_int / freq;
    ::core::ptr::write_volatile(
        (clk_regs.virt as uint32_t).wrapping_add(0xa0 as libc::c_int as uint32_t)
            as *mut uint32_t,
        (0x5a000000 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int) as uint32_t,
    );
    while *((clk_regs.virt as uint32_t).wrapping_add(0xa0 as libc::c_int as uint32_t)
        as *mut uint32_t) & ((1 as libc::c_int) << 7 as libc::c_int) as uint32_t != 0
    {}
    ::core::ptr::write_volatile(
        (clk_regs.virt as uint32_t).wrapping_add(0xa4 as libc::c_int as uint32_t)
            as *mut uint32_t,
        (0x5a000000 as libc::c_int | divi << 12 as libc::c_int) as uint32_t,
    );
    ::core::ptr::write_volatile(
        (clk_regs.virt as uint32_t).wrapping_add(0xa0 as libc::c_int as uint32_t)
            as *mut uint32_t,
        (0x5a000000 as libc::c_int | 6 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int) as uint32_t,
    );
    while *((clk_regs.virt as uint32_t).wrapping_add(0xa0 as libc::c_int as uint32_t)
        as *mut uint32_t) & ((1 as libc::c_int) << 7 as libc::c_int) as uint32_t
        == 0 as libc::c_int as uint32_t
    {}
    usleep(100 as libc::c_int as libc::c_uint);
    ::core::ptr::write_volatile(
        (pwm_regs.virt as uint32_t).wrapping_add(0x10 as libc::c_int as uint32_t)
            as *mut uint32_t,
        range as uint32_t,
    );
    ::core::ptr::write_volatile(
        (pwm_regs.virt as uint32_t).wrapping_add(0x18 as libc::c_int as uint32_t)
            as *mut uint32_t,
        val as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn start_pwm() {
    ::core::ptr::write_volatile(
        (pwm_regs.virt as uint32_t).wrapping_add(0 as libc::c_int as uint32_t)
            as *mut uint32_t,
        ((1 as libc::c_int) << 5 as libc::c_int | 1 as libc::c_int) as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn stop_pwm() {
    if !(pwm_regs.virt).is_null() {
        ::core::ptr::write_volatile(
            (pwm_regs.virt as uint32_t).wrapping_add(0 as libc::c_int as uint32_t)
                as *mut uint32_t,
            0 as libc::c_int as uint32_t,
        );
        usleep(100 as libc::c_int as libc::c_uint);
    }
}
