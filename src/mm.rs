use crate::mork_riscv::*;

pub type PageTableEntryImpl = page_table::PageTableEntryImpl;

pub type PageTableImpl = page_table::PageTableImpl;

pub fn vm_fence() {
    unsafe {
        page_table::vm_fence()
    }
}