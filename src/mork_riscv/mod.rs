#![allow(unused)]
#![no_std]

use core::arch::{asm, global_asm};
use log::{debug, info, warn};
pub mod sbi;
pub(crate) mod page_table;
pub(crate) mod config;
pub(crate) mod context;

global_asm!(include_str!("start.asm"));

pub fn idle_thread() {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}