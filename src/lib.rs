#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod consts;
mod error;
mod smileds;
mod rpi_dma_utils;

use std::{os::raw::c_int};

use error::Error;

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    unsafe fn to_c(&self) -> smileds::color_t {
        smileds::color_t {
            component: smileds::C2RustUnnamed_9 {
                r: self.r,
                g: self.g,
                b: self.b,
                a: self.a,
            },
        }
    }
}

#[derive(Debug)]
pub struct Ws2811;

impl Ws2811 {
    pub fn new(led_count: usize) -> Result<Self, Error> {
        unsafe { smileds::leds_init(led_count as c_int) };
        // let buffer = unsafe { from_raw_parts(leds_get_buffer(), TX_BUFF_LEN!(CHAN_MAXLEDS) as usize) };

        // Ok(Self { buffer })
        Ok(Self)
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        unsafe { smileds::leds_clear() };
        Ok(())
    }

    pub fn send(&mut self) -> Result<(), Error> {
        unsafe { smileds::leds_send() };
        Ok(())
    }

    pub fn set_pixel(&mut self, channel: usize, pixel: usize, color: Rgba) -> Result<(), Error> {
        unsafe { smileds::leds_set_pixel(channel as u8, pixel as u16, color.to_c()) };

        Ok(())
    }
}
