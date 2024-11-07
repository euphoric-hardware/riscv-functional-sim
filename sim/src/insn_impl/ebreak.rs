use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn ebreak(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("ebreak");
    
    todo!();
}