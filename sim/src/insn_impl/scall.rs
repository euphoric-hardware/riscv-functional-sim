use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn scall(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("scall");
    
    Ok(cpu.pc + 4)
}