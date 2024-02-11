#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::ptr::write_volatile;

use libc::c_int;

use crate::consts::{
    BIT_NPULSES, CHAN_MAXLEDS, CLK_BASE, DMA_BASE, DMA_CHAN, GPIO_ALT1, GPIO_BASE, GPIO_IN,
    LED_D0_PIN, LED_NBITS, LED_NCHANS, LED_TX_OFFSET, PAGE_SIZE, REQUEST_THRESH, SMI_TIMING,
    TX_BUFF_LEN, TX_BUFF_SIZE, VC_MEM_SIZE,
};

use super::{
    dma_defs::{
        DMA_CB_SRCE_INC, DMA_DEST_DREQ, DMA_SMI_DREQ, DMA_WAIT_RESP, MEM_BUS_ADDR, REG_BUS_ADDR,
    },
    rpi_dma_utils::{
        clk_regs, dma_regs, enable_dma, gpio_mode, gpio_regs, map_periph, map_uncached_mem,
        start_dma, stop_dma, unmap_periph_mem, DMA_CB, MEM_MAP,
    },
    smi_defs::*,
};

extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn usleep(__useconds: __useconds_t) -> c_int;
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
pub static mut led_count: uint16_t = 0 as c_int as uint16_t;
#[no_mangle]
pub static mut txdata: *mut uint16_t = 0 as *const uint16_t as *mut uint16_t;
#[no_mangle]
pub static mut tx_buffer: [uint16_t; TX_BUFF_LEN!(CHAN_MAXLEDS)] =
    [0 as uint16_t; TX_BUFF_LEN!(CHAN_MAXLEDS)];
#[no_mangle]
pub static mut color_buffer: [[color_t; 512]; 16] = [[color_t { packed: 0 }; 512]; 16];

#[no_mangle]
pub unsafe extern "C" fn map_devices() {
    map_periph(&mut gpio_regs, GPIO_BASE as *mut _, PAGE_SIZE as c_int);
    map_periph(&mut dma_regs, DMA_BASE as *mut _, PAGE_SIZE as c_int);
    map_periph(&mut clk_regs, CLK_BASE as *mut _, PAGE_SIZE as c_int);
    map_periph(&mut smi_regs, SMI_BASE as *mut _, PAGE_SIZE as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fail(mut s: *mut libc::c_char) {
    log::error!("{}", *s);
    terminate(0 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn terminate(mut _sig: c_int) {
    log::info!("Closing");
    if !(gpio_regs.virt).is_null() {
        for i in 0..LED_NCHANS {
            gpio_mode((LED_D0_PIN + i) as _, GPIO_IN as _);
        }
    }
    if !(smi_regs.virt).is_null() {
        ::core::ptr::write_volatile(REG32!(smi_regs, SMI_CS), 0 as c_int as uint32_t);
    }
    stop_dma(DMA_CHAN as _);
    unmap_periph_mem(&mut vc_mem);
    unmap_periph_mem(&mut smi_regs);
    unmap_periph_mem(&mut dma_regs);
    unmap_periph_mem(&mut gpio_regs);
    // Don't just exit! This is a library, not a standalone program.
    // exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn init_smi(
    mut width: c_int,
    mut ns: c_int,
    mut setup: c_int,
    mut strobe: c_int,
    mut hold: c_int,
) {
    let mut divi: c_int = ns / 2 as c_int;
    smi_cs = REG32!(smi_regs, SMI_CS) as *mut SMI_CS_REG;
    smi_l = REG32!(smi_regs, SMI_L) as *mut SMI_L_REG;
    smi_a = REG32!(smi_regs, SMI_A) as *mut SMI_A_REG;
    smi_d = REG32!(smi_regs, SMI_D) as *mut SMI_D_REG;
    smi_dmc = REG32!(smi_regs, SMI_DMC) as *mut SMI_DMC_REG;
    smi_dsr = REG32!(smi_regs, SMI_DSR0) as *mut SMI_DSR_REG;
    smi_dsw = REG32!(smi_regs, SMI_DSW0) as *mut SMI_DSW_REG;
    smi_dcs = REG32!(smi_regs, SMI_DCS) as *mut SMI_DCS_REG;
    smi_dca = REG32!(smi_regs, SMI_DCA) as *mut SMI_DCA_REG;
    smi_dcd = REG32!(smi_regs, SMI_DCD) as *mut SMI_DCD_REG;
    {
        let mut value = smi_a.read_volatile();
        value.value = 0;
        smi_a.write_volatile(value);
    }
    {
        let mut value = smi_l.read_volatile();
        value.value = 0;
        smi_l.write_volatile(value);
    }
    {
        let mut value = smi_cs.read_volatile();
        value.value = 0;
        smi_cs.write_volatile(value);
    }
    {
        let mut value = smi_dca.read_volatile();
        value.value = 0;
        smi_dca.write_volatile(value);
    }
    {
        let mut value = smi_dcs.read_volatile();
        value.value = 0;
        smi_dcs.write_volatile(value);
    }
    {
        let mut value = smi_dsw.read_volatile();
        value.value = 0;
        smi_dsw.write_volatile(value);
    }
    {
        let mut value = smi_dsr.read_volatile();
        value.value = 0;
        smi_dsr.write_volatile(value);
    }
    if REG32!(clk_regs, CLK_SMI_DIV).read_volatile() != (divi << 12) as uint32_t {
        ((REG32!(clk_regs, CLK_SMI_CTL) as *mut *mut uint32_t).read_volatile())
            .write_volatile((CLK_PASSWD | (1 << 5)) as uint32_t);
        usleep(10 as __useconds_t);

        while ((REG32!(clk_regs, CLK_SMI_CTL) as *mut uint32_t).read_volatile() & (1 << 7)) != 0 {}
        usleep(10 as __useconds_t);
        ((REG32!(clk_regs, CLK_SMI_DIV) as *mut *mut uint32_t).read_volatile())
            .write_volatile((CLK_PASSWD | (divi << 12) as usize) as uint32_t);
        usleep(10 as __useconds_t);
        ((REG32!(clk_regs, CLK_SMI_CTL) as *mut *mut uint32_t).read_volatile())
            .write_volatile((CLK_PASSWD | 6 | (1 << 4)) as uint32_t);
        usleep(10 as __useconds_t);
        while ((REG32!(clk_regs, CLK_SMI_CTL) as *mut uint32_t).read_volatile() & (1 << 7)) == 0 {}
        usleep(100 as __useconds_t);
    }
    // TODO: write_volatile here everywhere
    if smi_cs.read_volatile().cs.seterr() != 0 {
        let mut value = smi_cs.read_volatile();
        value.cs.set_seterr(1);
        smi_cs.write_volatile(value);
    }
    {
        let mut value = smi_dsw.read_volatile();
        value.dsw.set_wsetup(setup as uint32_t);
        value.dsw.set_wstrobe(strobe as uint32_t);
        value.dsw.set_whold(hold as uint32_t);
        value.dsw.set_wwidth(width as uint32_t);
        smi_dsw.write_volatile(value);
    }
    {
        let mut value = smi_dsr.read_volatile();
        value.dsr.set_rsetup(setup as uint32_t);
        value.dsr.set_rstrobe(strobe as uint32_t);
        value.dsr.set_rhold(hold as uint32_t);
        value.dsr.set_rwidth(width as uint32_t);
        smi_dsr.write_volatile(value);
    }
    {
        let mut value = smi_dmc.read_volatile();
        value.dmc.set_panicw(8);
        value.dmc.set_panicr(8u32);
        value.dmc.set_reqw(REQUEST_THRESH as uint32_t);
        value.dmc.set_reqr(REQUEST_THRESH as uint32_t);
        smi_dmc.write_volatile(value);
    }
    for i in 0..LED_NCHANS {
        gpio_mode((LED_D0_PIN + i) as c_int, GPIO_ALT1 as c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn setup_smi_dma(mut mp: *mut MEM_MAP, mut nsamp: c_int) {
    let mut cbs: *mut DMA_CB = (*mp).virt as *mut DMA_CB;
    txdata = cbs.offset(1 as isize) as *mut uint16_t;
    {
        let mut value = smi_dmc.read_volatile();
        value.dmc.set_dmaen(1);
        smi_dmc.write_volatile(value);
    }
    {
        let mut value = smi_cs.read_volatile();
        value.cs.set_enable(1);
        value.cs.set_clear(1);
        value.cs.set_pxldat(1);
        value.cs.set_write(1);
        smi_cs.write_volatile(value);
    }
    {
        let mut value = smi_l.read_volatile();
        value
            .l
            .set_len((nsamp as usize).wrapping_mul(::core::mem::size_of::<uint16_t>()) as uint32_t);
        smi_l.write_volatile(value);
    }
    enable_dma(10);
    (*cbs.offset(0)).ti =
        (DMA_DEST_DREQ | (DMA_SMI_DREQ << 16 as c_int) | DMA_CB_SRCE_INC | DMA_WAIT_RESP)
            as uint32_t;
    (*cbs.offset(0)).tfr_len =
        (nsamp as usize).wrapping_mul(::core::mem::size_of::<uint16_t>()) as uint32_t;
    (*cbs.offset(0)).srce_ad = MEM_BUS_ADDR!((*mp), txdata);
    (*cbs.offset(0)).dest_ad = REG_BUS_ADDR!(smi_regs, SMI_D) as uint32_t;
}

#[no_mangle]
pub unsafe extern "C" fn start_smi(mut mp: *mut MEM_MAP) {
    let mut cbs: *mut DMA_CB = (*mp).virt as *mut DMA_CB;
    start_dma(mp, DMA_CHAN as c_int, &mut *cbs.offset(0), 0);
    {
        let mut value = smi_cs.read_volatile();
        value.cs.set_start(1 as c_int as uint32_t);
        smi_cs.write_volatile(value);
    }
}

#[no_mangle]
pub unsafe extern "C" fn swap_bytes(mut data: *mut libc::c_void, mut len: c_int) {
    let mut wp: *mut uint16_t = data as *mut uint16_t;
    len = (len + 1 as c_int) / 2 as c_int;
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 > 0 as c_int) {
            break;
        }
        *wp = (*wp).swap_bytes();
        wp = wp.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn leds_init(mut init_led_count: c_int) -> c_int {
    if init_led_count as usize > CHAN_MAXLEDS {
        log::error!("smileds: Error! Max {} leds supported!", CHAN_MAXLEDS);
    }
    led_count = init_led_count as uint16_t;
    map_devices();
    let width = if LED_NCHANS > 8 {
        SMI_16_BITS
    } else {
        SMI_8_BITS
    };
    init_smi(
        width as c_int,
        SMI_TIMING[0] as c_int,
        SMI_TIMING[1] as c_int,
        SMI_TIMING[2] as c_int,
        SMI_TIMING[3] as c_int,
    );
    map_uncached_mem(&mut vc_mem, VC_MEM_SIZE as c_int);
    setup_smi_dma(&mut vc_mem, TX_BUFF_LEN!(led_count as usize) as c_int);

    // initialize bit pattern
    let mut tx_offset: *mut uint16_t =
        &mut *tx_buffer.as_mut_ptr().offset(LED_TX_OFFSET!(0) as isize);
    for _ in 0..(led_count as usize * LED_NBITS) {
        // This will get compiled out
        if LED_NCHANS <= 8 {
            *tx_offset.offset(0) = 0xff;
        } else {
            *tx_offset.offset(0) = 0xffff;
        }
        *tx_offset.offset(1 as c_int as isize) = 0x00;
        *tx_offset.offset(2 as c_int as isize) = 0x00;
        tx_offset = tx_offset.offset(BIT_NPULSES as isize);
    }
    leds_clear();
    log::info!(
        "smileds: Setting {} LEDs per channel, {} channels",
        led_count,
        LED_NCHANS
    );
    true as c_int
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
    if pixel >= led_count {
        return;
    }

    if channel as usize > LED_NCHANS {
        return;
    }

    if color.component.a == 255 {
        color_buffer[channel as usize][pixel as usize] = color;
    } else {
        let old_a: uint8_t = (1 - color.component.a) as uint8_t;
        color.component.r = (color_buffer[channel as usize][pixel as usize].component.r * old_a
            / 255
            + color.component.r * color.component.a / 255) as uint8_t;
        color.component.g = (color_buffer[channel as usize][pixel as usize].component.g * old_a
            / 255
            + color.component.g * color.component.a / 255) as uint8_t;
        color.component.b = (color_buffer[channel as usize][pixel as usize].component.b * old_a
            / 255
            + color.component.b * color.component.a / 255) as uint8_t;
        color_buffer[channel as usize][pixel as usize] = color;
    }

    // For each bit of the 24-bit RGB values..
    let channel_on_mask: uint16_t = (1 << channel) as uint16_t;
    let channel_off_mask: uint16_t = !channel_on_mask;
    let mut rgb_mask: uint32_t = (1 << 23) as uint32_t;
    let mut tx_offset: *mut uint16_t = &mut *tx_buffer
        .as_mut_ptr()
        .offset(LED_TX_OFFSET!(pixel as usize) as isize);
    for _ in 0..LED_NBITS {
        // tx_offset[0] always 0xffff
        // tx_offset[1] is the actual bit
        let ref mut ptr = *tx_offset.offset(1);
        if color.packed & rgb_mask != 0 {
            *ptr = *ptr | channel_on_mask;
        } else {
            *ptr = *ptr & channel_off_mask;
        }
        // tx_offset[2] always 0x0000
        tx_offset = tx_offset.offset(BIT_NPULSES as isize);
        rgb_mask = rgb_mask >> 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn leds_clear() {
    let mut tx_offset: *mut uint16_t =
        &mut *tx_buffer.as_mut_ptr().offset(LED_TX_OFFSET!(0) as isize);
    for _ in 0..(led_count as usize * LED_NBITS) {
        // tx_offset[0] always 0xffff
        *tx_offset.offset(1) = 0x0000;
        // tx_offset[2] always 0x0000
        tx_offset = tx_offset.offset(BIT_NPULSES as isize);
    }

    color_buffer.fill([color_t { packed: 0 }; 512]);
    // memset(
    //     color_buffer.as_mut_ptr() as *mut libc::c_void,
    //     0,
    //     ::core::mem::size_of::<[[color_t; 512]; 16]>() as libc::c_ulong,
    // );
}

#[no_mangle]
pub unsafe extern "C" fn leds_send() {
    memcpy(
        txdata as *mut libc::c_void,
        tx_buffer.as_mut_ptr() as *const libc::c_void,
        TX_BUFF_SIZE!(led_count as usize) as libc::c_ulong,
    );
    start_smi(&mut vc_mem);
}
