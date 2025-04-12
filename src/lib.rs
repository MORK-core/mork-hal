#![no_std]
#![feature(linkage)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use mork_common::types::{ResultWithErr, ResultWithValue};
use mork_common::utils::alignas::{align_down, align_up};
use mork_common::mork_kernel_log;
use crate::device_tree::FDTParser;

mod mork_riscv;
pub mod console;
mod logging;
mod device_tree;
pub mod mm;
pub mod context;
pub mod config;
pub mod timer;
pub mod trap;

pub const KERNEL_OFFSET: usize = mork_riscv::config::KERNEL_OFFSET;

pub fn shutdown(failure: bool) -> ! {
    mork_riscv::sbi::shutdown(failure)
}

pub fn console_putchar(c: char) {
    mork_riscv::sbi::console_putchar(c as usize);
}

pub fn console_getchar() -> usize {
    mork_riscv::sbi::console_getchar()
}

pub use mork_riscv::idle_thread;
use crate::context::HALContext;

pub fn trap_init() {
    mork_riscv::trap::init();
}

pub fn return_user(contex: *const HALContext) {
    mork_riscv::trap::return_user(contex);
}

/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

pub fn get_memory_info() -> ResultWithValue<(usize, usize, usize)> {
    let (start, size) = FDTParser.get_memory_range()?;
    unsafe extern "C" {
        fn kernel_end();
    }
    Ok((start + KERNEL_OFFSET,
        align_up(kernel_end as usize, 4096),
        align_down(start + size, 4096) + KERNEL_OFFSET
    ))
}

pub fn get_root_task_region() -> Result<(usize, usize), String> {
    unsafe extern "C" {
        fn root_task_data_start();
        fn root_task_data_end();
    }
    let (start, end) = (root_task_data_start as usize, root_task_data_end as usize);
    if start >= end {
        Err(format!("Invaild Root task Region: {:#x} -- {:#x}", start, end))
    } else {
        Ok((start, end))
    }
}

pub fn init(dtb_paddr: usize) -> ResultWithErr<String> {
    clear_bss();
    logging::init();
    // info!("start device tree init");
    mork_kernel_log!(info, "start device tree init");
    device_tree::init(dtb_paddr)
}