use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_jal(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_jal", c_imm12 = insn.c_imm12());

    let c_imm12 = insn.c_imm12();

    todo!();
}