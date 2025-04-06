use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn nop_raw(cpu: &mut Cpu) -> cpu::Result<u64> {
    Ok(cpu.pc + 4)
}