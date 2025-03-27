use alloc::string::String;
use mork_common::types::{ResultWithErr, ResultWithValue};
use log::warn;
use fdt::Fdt;
use lazy_init::LazyInit;

static FDT: LazyInit<Fdt> = LazyInit::new();

pub struct FDTParser;

impl FDTParser {
    pub fn get_memory_range(&self) -> ResultWithValue<(usize, usize)> {
        if !FDT.is_init() {
            warn!("FDT is not initialized");
            return Err(());
        }
        for node in FDT.all_nodes() {
            if node.name.starts_with("memory@") {
                if let Some(reg) = node.property("reg") {
                    let reg_bytes = reg.value;
                    // 检查字节切片的长度
                    if reg_bytes.len() >= 16 {
                        // 解析起始地址和大小（假设每个部分为 64 位）
                        let start = u64::from_be_bytes([
                            reg_bytes[0], reg_bytes[1], reg_bytes[2], reg_bytes[3],
                            reg_bytes[4], reg_bytes[5], reg_bytes[6], reg_bytes[7],
                        ]) as usize;

                        let size = u64::from_be_bytes([
                            reg_bytes[8], reg_bytes[9], reg_bytes[10], reg_bytes[11],
                            reg_bytes[12], reg_bytes[13], reg_bytes[14], reg_bytes[15],
                        ]) as usize;
                        return Ok((start, size));
                    }
                }
            }
        }
        Err(())
    }
}


pub fn init(dtb_paddr: usize) -> ResultWithErr<String> {
    let dtb_ptr = dtb_paddr as *const u8;
    if dtb_ptr.is_null() {
        return Err("No valid DTB address provided!".into());
    }
    let dtb = unsafe { Fdt::from_ptr(dtb_ptr).expect("Failed to parse DTB") };
    FDT.init_by(dtb);
    Ok(())
}
