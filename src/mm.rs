use mork_capability::cap::PageTableCap;
use crate::mork_riscv::*;

pub type PageTableEntryImpl = page_table::PageTableEntryImpl;

pub type PageTableImpl = page_table::PageTableImpl;

impl PageTableImpl {
    pub fn from_cap(cap: &PageTableCap) -> &mut Self {
        unsafe {
            &mut *((cap.base_ptr() << 12) as usize as *mut Self)
        }
    }
}

pub fn vm_fence() {
    unsafe {
        page_table::vm_fence()
    }
}