use crate::cpu::{Cpu, Insn};

pub fn sbreak(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("sbreak");
    
    todo!();
}