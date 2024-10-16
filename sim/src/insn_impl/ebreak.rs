use crate::cpu::{Cpu, Insn};

pub fn ebreak(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("ebreak");
    
    todo!();
}