#![allow(unused)]
#![no_std]

use core::arch::global_asm;
use log::{debug, info, warn};
pub mod sbi;
mod page_table;

pub const KERNEL_OFFSET: usize = 0xFFFFFFFF_00000000;

global_asm!(include_str!("start.asm"));

/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}