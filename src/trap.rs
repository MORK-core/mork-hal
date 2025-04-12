#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ExceptionCause {
    IllegalInstruction,
    LoadPageFault,
    StorePageFault,
    InstructionPageFault,
    Unknown
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum InterruptCause {
    UserSoft,
    SupervisorSoft,
    UserTimer,
    SupervisorTimer,
    UserExternal,
    SupervisorExternal
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum TrapCause {
    Exception(ExceptionCause),
    Interrupt(InterruptCause),
    Unknown,
}