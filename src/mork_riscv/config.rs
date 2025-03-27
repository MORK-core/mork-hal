pub const PAGE_TABLE_INDEX_BITS: usize = 9;
pub const ROOT_PAGE_TABLE_SIZE: usize = 1 << PAGE_TABLE_INDEX_BITS;
pub const NORMAL_PAGE_BITS: usize = 12;
pub const NORMAL_PAGE_SIZE: usize = 1 << NORMAL_PAGE_BITS;
pub const LEVEL2_PAGE_BITS: usize = NORMAL_PAGE_BITS + 9;
pub const LEVEL2_PAGE_SIZE: usize = 1 << LEVEL2_PAGE_BITS;
pub const LEVEL1_PAGE_BITS: usize = LEVEL2_PAGE_BITS + 9;
pub const LEVEL1_PAGE_SIZE: usize = 1 << LEVEL1_PAGE_BITS;

pub const CONTEXT_REGISTERS_NUM: usize = 35;

pub const PAGE_LEVEL: usize = 3;
