use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn wfi_raw(cpu: &mut Cpu) -> cpu::Result<u64> {
    Ok(cpu.pc)
}