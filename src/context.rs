use crate::mork_riscv;

pub type HALContext = mork_riscv::context::Context;

pub trait HALContextTrait {
    fn new() -> Self;

    fn set_stack(&mut self, stack_ptr: usize);

    fn set_next_ip(&mut self, next_ip: usize);

    fn configure_idle(&mut self);
}