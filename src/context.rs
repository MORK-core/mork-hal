use mork_common::syscall::message_info::MessageInfo;
use crate::mork_riscv;

pub type HALContext = mork_riscv::context::Context;

pub trait HALContextTrait {
    fn new() -> Self;

    fn get_pointer(&self) -> *const Self;

    fn set_stack(&mut self, stack_ptr: usize);

    fn set_next_ip(&mut self, next_ip: usize);
    fn get_next_ip(&self) -> usize;

    fn set_user_flag(&mut self, is_user: bool);

    fn set_interrupt_enable(&mut self, enable: bool);

    fn get_cap(&self) -> usize;

    fn get_tag(&self) -> MessageInfo;

    fn set_tag(&mut self, tag: MessageInfo);

    fn get_fault_ip(&self) -> usize;

    fn set_mr(&mut self, idx: usize, value: usize);

    fn get_mr(&self, idx: usize) -> usize;

    fn set_tls_base(&mut self, base: usize);

    fn set_badge(&mut self, badge: usize);
}