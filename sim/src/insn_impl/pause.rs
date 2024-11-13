use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn pause(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("pause");
    
    Ok(cpu.pc + 4)
}