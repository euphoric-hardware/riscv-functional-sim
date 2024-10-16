use crate::cpu::{Cpu, Insn};

pub fn ret(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("ret");
    
    todo!();
}