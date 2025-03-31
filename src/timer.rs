use crate::mork_riscv;

pub fn set_next_trigger() {
    mork_riscv::timer::set_next_trigger();
}