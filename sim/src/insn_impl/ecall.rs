use crate::cpu::{Cpu, Insn};

pub fn ecall(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("ecall");
    
    todo!();
}