use crate::{
    cpu::{self, Cpu, Insn},
    uop_cache::UopCacheEntry,
};

pub fn sltu_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let result = if cpu.load(rs1) < cpu.load(rs2) {
        1
    } else {
        0
    };
    cpu.store(rd, result);
    Ok(cpu.pc + 4)
}
