use core::ops::{Index, IndexMut};
use mork_common::syscall::message_info::MessageInfo;
use mork_common::types::Array;
use crate::context::HALContextTrait;
use crate::mork_riscv::register::{Register, BADGE_REGISTER, MESSAGE_REGISTERS, SSTATUS_SPIE, SSTATUS_SPP};

pub struct Context {
    registers: Array<usize, 35>,
}

impl Index<Register> for Context {
    type Output = usize;

    fn index(&self, index: Register) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<Register> for Context {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl Index<usize> for Context {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.registers[index]
    }
}

impl IndexMut<usize> for Context {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.registers[index]
    }
}

impl HALContextTrait for Context {
    fn new() -> Context {
        Self {
            registers: Array::default(),
        }
    }

    fn get_pointer(&self) -> *const Self {
        self as *const Self
    }

    fn set_stack(&mut self, stack_ptr: usize) {
        self[Register::sp] = stack_ptr;
    }

    fn set_next_ip(&mut self, next_ip: usize) {
        self[Register::NextIP] = next_ip;
    }

    fn get_next_ip(&self) -> usize {
        self[Register::NextIP]
    }

    fn set_user_flag(&mut self, is_user: bool) {
        if is_user {
            self[Register::SSTATUS] &= !SSTATUS_SPP;
        } else {
            self[Register::SSTATUS] |= SSTATUS_SPP;
        }
    }

    fn set_interrupt_enable(&mut self, enable: bool) {
        if enable {
            self[Register::SSTATUS] |= SSTATUS_SPIE;
        } else {
            self[Register::SSTATUS] &= !SSTATUS_SPIE;
        }
    }

    fn get_cap(&self) -> usize {
        self[Register::a0]
    }

    fn get_tag(&self) -> MessageInfo {
        MessageInfo::from_word(self[Register::a1])
    }

    fn set_tag(&mut self, tag: MessageInfo) {
        self[Register::a1] = tag.to_word();
    }

    fn get_fault_ip(&self) -> usize {
        self[Register::FaultIP]
    }

    fn set_mr(&mut self, idx: usize, value: usize) {
        self[MESSAGE_REGISTERS[idx]] = value;
    }

    fn get_mr(&self, idx: usize) -> usize {
        self[MESSAGE_REGISTERS[idx]]
    }

    fn set_tls_base(&mut self, base: usize) {
        self[Register::tp] =  base;
    }

    fn set_badge(&mut self, badge: usize) {
        self[BADGE_REGISTER] = badge;
    }
}
