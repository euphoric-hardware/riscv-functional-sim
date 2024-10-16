use crate::cpu::{Cpu, Insn};

pub fn scall(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("scall");
    
    todo!();
}