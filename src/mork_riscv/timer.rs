use riscv::register::{sie, time};
use crate::mork_riscv::config::{CLOCK_FREQ, TICKS_PER_SEC};
use crate::mork_riscv::sbi::set_timer;

pub fn init() {
    unsafe {
        sie::set_stimer();
    }
}

pub fn get_time() -> usize {
    time::read()
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC)
}