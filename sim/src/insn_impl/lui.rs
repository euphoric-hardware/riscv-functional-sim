use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn lui(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("lui", rd = insn.rd(), imm20 = insn.imm20());

    let rd = insn.rd();
    let imm20 = insn.imm20();

    todo!();
}