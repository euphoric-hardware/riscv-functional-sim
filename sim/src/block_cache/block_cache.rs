use crate::{
    cpu::Cpu,
    uop_cache::uop_cache::UopCacheEntry
};

#[derive(Default, Debug)]
pub struct BlockCacheEntry {
    start_pc: u64,
}


impl BlockCacheEntry {
    pub fn new(cpu: Cpu, start_pc: u64, end_pc: u64) -> Self {
        let mut entry = BlockCacheEntry::default();
        entry.start_pc = start_pc;
        entry
    }
}