use core::arch::{asm, global_asm};
use log::debug;
use mork_common::mork_kernel_log;

global_asm!(include_str!("trap.asm"));
use riscv::register::stvec::TrapMode;
use riscv::register::{scause, stval, stvec};
use riscv::register::scause::{Exception, Interrupt, Trap};
use crate::mork_riscv::context::Context;
use crate::mork_riscv::sbi::shutdown;
use crate::mork_riscv::timer;

pub fn init() {
    unsafe extern "C" {
        fn trap_entry();
    }

    unsafe {
        stvec::write(trap_entry as usize, TrapMode::Direct);
    }
    timer::init();
}

pub fn return_user(contex: *const Context) {
    // mork_kernel_log!(debug, "return_user::return_user()");
    let base_ptr = contex as usize;
    unsafe {
        asm!(
        "mv t0, {0}",
        "ld ra, (0*8)(t0)",
        "ld sp, (1*8)(t0)",
        "ld gp, (2*8)(t0)",
        // skip tp
        // skip t0
        // no-op store conditional to clear monitor state
        // this may succeed in implementations with very large reservations, but the saved ra is dead
        "sc.d zero, zero, (t0)",
        "ld t2, (6*8)(t0)",
        "ld s0, (7*8)(t0)",
        "ld s1, (8*8)(t0)",
        "ld a0, (9*8)(t0)",
        "ld a1, (10*8)(t0)",
        "ld a2, (11*8)(t0)",
        "ld a3, (12*8)(t0)",
        "ld a4, (13*8)(t0)",
        "ld a5, (14*8)(t0)",
        "ld a6, (15*8)(t0)",
        "ld a7, (16*8)(t0)",
        "ld s2, (17*8)(t0)",
        "ld s3, (18*8)(t0)",
        "ld s4, (19*8)(t0)",
        "ld s5, (20*8)(t0)",
        "ld s6, (21*8)(t0)",
        "ld s7, (22*8)(t0)",
        "ld s8, (23*8)(t0)",
        "ld s9, (24*8)(t0)",
        "ld s10, (25*8)(t0)",
        "ld s11, (26*8)(t0)",
        "ld t3, (27*8)(t0)",
        "ld t4, (28*8)(t0)",
        "ld t5, (29*8)(t0)",
        "ld t6, (30*8)(t0)",
        // Get next restored tp
        "ld t1, (3*8)(t0)",
        // get restored tp
        "add tp, t1, x0",
        // get sepc
        "ld t1, (34*8)(t0)",
        "csrw sepc, t1",
        // Write back sscratch with cur_thread_reg to get it back on the next trap entry
        "csrw sscratch, t0",
        "ld t1, (32*8)(t0)",
        "csrw sstatus, t1",
        "ld t1, (5*8)(t0)",
        "ld t0, (4*8)(t0)",
        "sret",
            in(reg) base_ptr,
        );
    }
    assert_eq!(1, 0);
}

#[unsafe(no_mangle)]
pub fn handle_interrupt() {
    let scause = scause::read();
    let stval = stval::read();
    debug!("scause={:?}, stval={:#x}", scause.cause(), stval);
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            mork_kernel_log!(info, "receive timer interrupt");
            shutdown(false)
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            mork_kernel_log!(error,
                "{:?} in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.",
                scause.cause(),
                stval,
                0,
            );
            shutdown(true)
        }
        _ => {
            panic!("invalid interrupt");
        }
    }
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
pub fn handle_exception() {
    // panic!("invalid exception");
    handle_interrupt();
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
pub fn handle_syscall(cptr: usize, msg_info: usize, syscall: isize) {
    panic!("syscall handler invalid");
}