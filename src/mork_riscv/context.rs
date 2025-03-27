use mork_common::types::Array;
use crate::context::HALContextTrait;

pub struct Context {
    registers: Array<usize, 35>,
}

impl HALContextTrait for Context {
    fn new() -> Context {
        Self {
            registers: Array::default(),
        }
    }

    fn set_stack(&mut self, stack_ptr: usize) {
        todo!()
    }

    fn set_next_ip(&mut self, next_ip: usize) {
        todo!()
    }

    fn configure_idle(&mut self) {
        todo!()
    }
}
