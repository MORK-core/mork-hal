#![no_std]

use log::debug;
use common::utils::alignas::{align_down, align_up};
use crate::device_tree::FDTParser;

mod mork_riscv;
pub mod console;
mod logging;
mod lang_items;
mod device_tree;
mod page_table;

const KERNEL_OFFSET: usize = mork_riscv::KERNEL_OFFSET;

pub fn shutdown(failure: bool) -> ! {
    mork_riscv::sbi::shutdown(failure)
}

pub fn console_putchar(c: char) {
    mork_riscv::sbi::console_putchar(c as usize);
}

pub fn console_getchar() -> usize {
    mork_riscv::sbi::console_getchar()
}

pub fn get_free_memory() -> Result<(usize, usize), ()> {
    let (start, size) = FDTParser.get_memory_range()?;
    unsafe extern "C" {
        fn kernel_end();
    }
    Ok((align_up(kernel_end as usize, 4096), align_down(start + size + KERNEL_OFFSET, 4096)))
}

pub fn boot_init(dtb_paddr: usize) {
    mork_riscv::clear_bss();
    logging::init();
    device_tree::parse_dtb(dtb_paddr + KERNEL_OFFSET);
}