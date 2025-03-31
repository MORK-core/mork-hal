use crate::mork_riscv;

pub type HALContext = mork_riscv::context::Context;

pub trait HALContextTrait {
    fn new() -> Self;

    fn get_pointer(&self) -> *const Self;

    fn set_stack(&mut self, stack_ptr: usize);

    fn set_next_ip(&mut self, next_ip: usize);

    fn set_user_flag(&mut self, is_user: bool);

    fn set_interrupt_enable(&mut self, enable: bool);

    fn get_cap(&self) -> usize;
}