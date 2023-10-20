mod dma;
mod error;
mod mbox;
mod smi;
mod utils;

use std::{ffi::c_void, fs::File, iter::repeat};

use dma::DMA_MEM_FLAGS;
use error::Error;
use libc::memcpy;
use mbox::{open_mbox, VcMap};
use memmap2::MmapRaw;
use smi::Smi;
use utils::page_roundup;

#[cfg(rpi4)]
const SMI_TIMING: [u8; 4] = [10, 15, 30, 15];
#[cfg(not(rpi4))]
const SMI_TIMING: [u8; 4] = [10, 10, 20, 10];

/// GPIO pin for D0 output
const LED_D0_PIN: u8 = 8;
/// Number of LED channels (8 or 16)
const LED_NCHANS: usize = 16;
/// Number of bits per LED
const LED_NBITS: usize = 24;
/// Number of zero bits before LED data
const LED_PREBITS: usize = 4;
/// Number of zero bits after LED data
const LED_POSTBITS: usize = 4;
/// Number of O/P pulses per LED bit
const BIT_NPULSES: usize = 3;
/// Max number of LEDs per channel
const CHAN_MAXLEDS: usize = 512;
/// DMA request threshold
const REQUEST_THRESH: usize = 2;

const GPIO_ALT1: u8 = 5;

const LED_DLEN: usize = LED_NBITS * BIT_NPULSES;

#[cfg(rpi4)]
const PHYS_REG_BASE: *mut c_void = 0xFE000000 as *mut c_void;
#[cfg(rpi23)]
const PHYS_REG_BASE: *mut c_void = 0x3F000000 as *mut c_void;
#[cfg(not(any(rpi23, rpi4)))]
const PHYS_REG_BASE: *const c_void = 0x20000000 as *const c_void;

const GPIO_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x200000);
const DMA_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x007000);
const CLK_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x101000);
const SMI_BASE: *const c_void = PHYS_REG_BASE.wrapping_offset(0x600000);
const BUS_REG_BASE: *mut c_void = 0x7E000000 as *mut c_void;
const PAGE_SIZE: usize = 0x1000;

// TODO: improve this structure for rust
#[repr(C)]
/// Structure for mapped peripheral or memory
#[derive(Debug)]
struct MemMap {
    /// Memory size
    size: usize,
    /// Bus address
    bus: *mut c_void,
    /// Virtual address
    virt: Option<MmapRaw>,
    /// Physical address
    phys: *mut c_void,
}

impl Default for MemMap {
    fn default() -> Self {
        Self {
            size: 0,
            bus: std::ptr::null_mut(),
            virt: None,
            phys: std::ptr::null_mut(),
        }
    }
}

impl MemMap {
    fn map_periph(phys: *mut c_void, size: usize) -> Result<Self, Error> {
        let size = utils::page_roundup(size);
        Ok(Self {
            size,
            phys,
            bus: (phys as usize - PHYS_REG_BASE as usize + BUS_REG_BASE as usize) as *mut c_void,
            virt: Some(utils::map_segment(phys, size)?),
        })
    }
}

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

const VC_MEM_SIZE: usize = PAGE_SIZE + TX_BUFF_SIZE!(CHAN_MAXLEDS);

#[derive(Debug)]
pub struct Ws2811 {
    /// GPIO registers
    gpio_regs: MemMap,
    /// Clock registers
    clk_regs: MemMap,
    /// DMA registers
    dma_regs: MemMap,
    /// VideoCore memory
    vc_mem: Option<VcMap>,
    /// SMI registers
    smi_regs: MemMap,

    smi: Smi,

    /// Used number of leds
    led_count: usize,
    /// Pointer to uncached Tx data buffer
    // TODO: make this a reference
    tx_data: *mut u16,
    /// Tx buffer for assembling data
    tx_buffer: [u16; TX_BUFF_LEN!(CHAN_MAXLEDS)],
    color_buffer: [[Rgba; CHAN_MAXLEDS]; LED_NCHANS],
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for Ws2811 {
    fn default() -> Self {
        Self {
            gpio_regs: Default::default(),
            clk_regs: Default::default(),
            dma_regs: Default::default(),
            smi_regs: Default::default(),
            vc_mem: Default::default(),
            smi: Default::default(),

            led_count: 0,
            tx_data: std::ptr::null_mut(),
            tx_buffer: [0; TX_BUFF_LEN!(CHAN_MAXLEDS)],
            color_buffer: [[Default::default(); LED_NCHANS]; CHAN_MAXLEDS],
        }
    }
}

// TODO: Define volatile SMI registers

impl Ws2811 {
    fn map_devices(&mut self) -> Result<(), Error> {
        self.gpio_regs = MemMap::map_periph(GPIO_BASE as _, PAGE_SIZE)?;
        self.dma_regs = MemMap::map_periph(DMA_BASE as _, PAGE_SIZE)?;
        self.clk_regs = MemMap::map_periph(CLK_BASE as _, PAGE_SIZE)?;
        self.smi_regs = MemMap::map_periph(SMI_BASE as _, PAGE_SIZE)?;
        Ok(())
    }

    pub unsafe fn new(led_count: usize) -> Result<Self, Error> {
        let mut this = Self::default();
        this.led_count = led_count;
        unsafe {
            this.map_devices()?;
            this.smi.init(
                &this.smi_regs,
                &this.clk_regs,
                &this.gpio_regs,
                smi::SMI_16_BITS,
                SMI_TIMING,
            )?;
            this.vc_mem = Some(VcMap::map_uncached_mem(VC_MEM_SIZE)?);
            this.vc_mem
                .as_ref()
                .ok_or(Error::VcMemUninitialized)?
                .setup_smi_dma(
                    &mut this.tx_data,
                    &this.smi,
                    &this.smi_regs,
                    &this.dma_regs,
                    TX_BUFF_LEN!(led_count),
                )?;

            // initalize bit pattern
            for (buff, other) in this.tx_buffer[(LED_TX_OFFSET!(0))..][..led_count * LED_NBITS]
                .iter_mut()
                .zip([0xffffu16, 0x00, 0x00].into_iter().cycle())
            {
                *buff = other;
            }

            this.clear()?;
        }
        log::info!(
            "smileds: Setting {} LEDs per channel, {} channels",
            led_count,
            LED_NCHANS
        );

        Ok(this)
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        for (_, buff) in self.tx_buffer[(LED_TX_OFFSET!(0))..][..self.led_count * LED_NBITS]
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| i % 3 == 1)
        {
            *buff = 0x0000;
        }

        self.color_buffer = [[Default::default(); LED_NCHANS]; CHAN_MAXLEDS];
        Ok(())
    }

    pub fn send(&mut self) -> Result<(), Error> {
        unsafe {
            memcpy(
                self.tx_data as *mut c_void,
                self.tx_buffer.as_ptr() as *mut c_void,
                TX_BUFF_SIZE!(self.led_count),
            );
            self.vc_mem
                .as_ref()
                .expect("Should call this after init")
                .start_smi(&self.smi)?;
        }
        Ok(())
    }

    pub fn set_pixel(&mut self, channel: usize, pixel: usize, color: Rgba) -> Result<(), Error> {
        if pixel >= self.led_count {
            return Err(Error::InvalidLed);
        }

        if channel >= LED_NCHANS {
            return Err(Error::InvalidChannel);
        }

        // TODO: alpha blending
        self.color_buffer[channel][pixel] = color;

        let channel_on_mask = 1 << channel;
        let channel_off_mask = !channel_on_mask;
        let rgb_mask = 1 << 23;
        let tx_offset = &self.tx_buffer[LED_TX_OFFSET!(pixel)];
    
        Ok(())
    }
}
