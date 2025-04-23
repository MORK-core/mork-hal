use core::ops::{Index, IndexMut};
use bitflags::bitflags;
use mork_common::types::{Array, JustResult, ResultWithValue};
use mork_common::utils::alignas::{align_down, align_up, is_aligned};
use log::{debug, warn};
use mork_common::mork_kernel_log;
use crate::KERNEL_OFFSET;
use crate::mork_riscv::config::{LEVEL1_PAGE_SIZE, LEVEL2_PAGE_SIZE, NORMAL_PAGE_BITS, NORMAL_PAGE_SIZE, ROOT_PAGE_TABLE_SIZE};

pub unsafe fn vm_fence() {
    unsafe {
        riscv::asm::sfence_vma_all();
    }
}

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C, align(8))]
pub struct PageTableEntryImpl {
    bits: usize,
}

impl PageTableEntryImpl {
    pub fn new_for_kernel(ppn: usize, is_leaf: bool) -> Self {
        let mut flag = PTEFlags::G | PTEFlags::V;
        if is_leaf {
            flag = flag | PTEFlags::R |PTEFlags::W | PTEFlags::X | PTEFlags::D | PTEFlags::A;
        }
        PageTableEntryImpl {
            bits: ppn << 10 | flag.bits as usize,
        }
    }

    pub fn new_for_user_frame(ppn: usize, is_x: bool, is_w: bool, is_r: bool) -> Self {
        let mut flag = PTEFlags::V | PTEFlags::U | PTEFlags::A;
        if is_x {
            flag = flag | PTEFlags::X;
        }
        if is_w {
            flag = flag | PTEFlags::W;
        }
        if is_r {
            flag = flag | PTEFlags::R;
        }
        PageTableEntryImpl {
            bits: ppn << 10 | flag.bits as usize,
        }
    }

    pub fn new_for_user_page_table(ppn: usize) -> Self {
        let mut flag = PTEFlags::V;
        PageTableEntryImpl {
            bits: ppn << 10 | flag.bits as usize,
        }
    }

    pub unsafe fn get_page_table(&self) -> &'static mut PageTableImpl {
        let vaddr = (((self.bits >> 10) & 0xFFFFF) << 12) + KERNEL_OFFSET;
        unsafe {
            &mut *(vaddr as *mut PageTableImpl)
        }
    }

    pub fn is_leaf(&self) -> bool {
        let flag = (self.bits & 0xFF) as u8;
        flag & PTEFlags::R.bits != 0
            || flag & PTEFlags::W.bits != 0
            || flag & PTEFlags::X.bits != 0
    }

    pub fn get_ppn(&self) -> usize {
        assert!(self.is_leaf());
        (self.bits >> 10) & 0x000FFFFFFFFFFFFF
    }

    pub fn valid(&self) -> bool {
        self.bits & PTEFlags::V.bits as usize != 0
    }
}

#[repr(C, align(4096))]
#[derive(Copy, Clone)]
pub struct PageTableImpl {
    table: [PageTableEntryImpl; ROOT_PAGE_TABLE_SIZE],
}

impl PageTableImpl {
    pub fn new() -> Self {
        PageTableImpl {
            table: [PageTableEntryImpl::default(); ROOT_PAGE_TABLE_SIZE],
        }
    }

    pub fn active(&self) {
        let ppn = (self as *const Self as usize - KERNEL_OFFSET) >> NORMAL_PAGE_BITS;
        use riscv::register::satp;
        unsafe {
            satp::set(satp::Mode::Sv39, 0, ppn);
            vm_fence();
        }
    }

    pub fn get_pte(&self, index: usize) -> PageTableEntryImpl {
        self.table[index].clone()
    }

    pub fn get_ptr(&self) -> usize {
        self as *const PageTableImpl as usize
    }

    pub fn map_page_table(&mut self, vaddr: usize, page_table_addr: usize, level: usize) {
        let index = Self::get_index(vaddr, level).unwrap();
        self.map_page_table_for_user(page_table_addr, index);
    }

    pub fn map_frame_for_user(&mut self, vaddr: usize, paddr: usize, level: usize,
                              is_x: bool, is_w: bool, is_r: bool) {
        let index = Self::get_index(vaddr, level).unwrap();
        self.map_page_for_user(paddr, index, is_x, is_w, is_r);
    }

    pub fn unmap_frame(&mut self, vaddr: usize, level: usize) {
        let index = Self::get_index(vaddr, level).unwrap();
        self.unmap_page(index);
    }

    pub fn map_frame_for_kernel(&mut self, vaddr: usize, paddr: usize, level: usize) {
        let index = Self::get_index(vaddr, level).unwrap();
        self.map_page_for_kernel(paddr, index);
    }


    pub fn get_index(vaddr: usize, level: usize) -> Option<usize> {
        match level {
            0 => Some((vaddr >> 30) & 0x1FF),
            1 => Some((vaddr >> 21) & 0x1FF),
            2 => Some((vaddr >> 12) & 0x1FF),
            _ => None
        }
    }

    pub fn get_align(level: usize) -> Option<usize> {
        match level {
            3 => Some(NORMAL_PAGE_SIZE),
            2 => Some(LEVEL2_PAGE_SIZE),
            1 => Some(LEVEL1_PAGE_SIZE),
            _ => None
        }
    }

    pub fn get_size(level: usize) -> Option<usize> {
        match level {
            0 => Some(LEVEL1_PAGE_SIZE),
            1 => Some(LEVEL2_PAGE_SIZE),
            2 => Some(NORMAL_PAGE_SIZE),
            _ => None
        }
    }

    fn map_page_for_kernel(&mut self, mut paddr: usize, index: usize) {
        self.table[index] = PageTableEntryImpl::new_for_kernel(paddr >> 12, true);
    }

    fn map_page_for_user(&mut self, mut paddr: usize, index: usize,
                         is_x: bool, is_w: bool, is_r: bool) {
        self.table[index] = PageTableEntryImpl::new_for_user_frame(
            paddr >> 12, is_x, is_w, is_r
        );
    }

    fn unmap_page(&mut self, index: usize) {
        self.table[index] = PageTableEntryImpl::default();
    }

    fn map_page_table_for_user(&mut self, mut paddr: usize, index: usize) {
        self.table[index] = PageTableEntryImpl::new_for_user_page_table(paddr >> 12);
    }
}


impl Index<usize> for PageTableImpl {
    type Output = PageTableEntryImpl;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}

impl IndexMut<usize> for PageTableImpl {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.table[index]
    }
}