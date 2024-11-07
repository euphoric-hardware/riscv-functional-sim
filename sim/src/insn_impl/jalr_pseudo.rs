use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn jalr_pseudo(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("jalr_pseudo", rs1 = insn.rs1());

    let rs1 = insn.rs1();

    todo!();
}