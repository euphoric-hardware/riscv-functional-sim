use super::parse_opcodes_out::*;
use crate::{
    cpu::{Cpu, Insn},
    insn_impl,
};

impl Cpu {
    pub fn execute(&mut self, insn: Insn) {
        let bits = insn.bits();

        if bits as u32 & MASK_ADD == MATCH_ADD {
            insn_impl::add::add(insn, self);
        } else {
            panic!("unknown instruction!")
        }
    }
}
