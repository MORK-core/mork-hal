pub trait PageTableEntry {
    fn new()-> Self;
    fn modify(&mut self, addr: usize, size: usize);
    fn is_leaf(&self) -> bool;
    fn is_valid(&self) -> bool;
    fn is_user(&self) -> bool;
    fn is_readable(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn is_executable(&self) -> bool;
    fn is_modified(&self) -> bool;
    fn get_ppn(&self) -> usize;
    fn set_ppn(&mut self, ppn: usize);
    fn get_next(&mut self)-> dyn PageTable;
}

struct PageTableEntryImpl {

}

pub trait PageTable {
    fn is_contain(&self, addr: usize)-> bool;
}