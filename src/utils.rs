use std::{fs::File, os::unix::prelude::OpenOptionsExt};

use memmap2::{MmapOptions, MmapRaw};

use crate::{error::Error, smi::REG32, MemMap, PAGE_SIZE};

#[inline(always)]
pub(crate) fn page_roundup(size: u32) -> u32 {
    if size % PAGE_SIZE == 0 {
        size
    } else {
        (size + PAGE_SIZE) & !(PAGE_SIZE - 1)
    }
}

pub(crate) fn map_segment(phys: *mut std::ffi::c_void, size: u32) -> Result<MmapRaw, Error> {
    let size = page_roundup(size);
    let file = File::options()
        .read(true)
        .write(true)
        .custom_flags(libc::O_RDWR | libc::O_SYNC)
        .open("/dev/mem")?;
    let map = MmapOptions::new()
        .offset(phys as u64)
        .len(size as usize)
        .map_raw(&file)?;
    Ok(map)
}

pub(crate) unsafe fn gpio_mode(gpio_regs: &MemMap, pin: u8, mode: u32) -> Result<(), Error> {
    // volatile uint32_t *reg = REG32(gpio_regs, GPIO_MODE0) + pin / 10, shift = (pin % 10) * 3;
    #[allow(unused_unsafe)]
    let reg = unsafe { REG32!(gpio_regs, 0).offset(pin as isize / 10) };
    let shift = pin % 10 * 3;

    // *reg = (*reg & ~(7 << shift)) | (mode << shift);
    unsafe { reg.write_volatile(reg.read_volatile() & !(7 << shift) | (mode << shift) as u32) };
    Ok(())
}
