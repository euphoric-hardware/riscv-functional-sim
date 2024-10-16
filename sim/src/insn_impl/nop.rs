use crate::cpu::{Cpu, Insn};

pub fn nop(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("nop");
    
    todo!();
}