use crate::mork_riscv;

pub const HAL_PAGE_LEVEL: usize = mork_riscv::config::PAGE_LEVEL;

pub const PAGE_SIZE_NORMAL: usize = mork_riscv::config::NORMAL_PAGE_SIZE;

pub const PAGE_SIZE_2M: usize = mork_riscv::config::LEVEL2_PAGE_SIZE;