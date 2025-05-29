use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn fence_raw(cpu: &mut Cpu) -> cpu::Result<u64> {
    Ok(cpu.pc + 4)
}