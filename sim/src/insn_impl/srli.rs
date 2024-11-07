use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn srli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("srli", rd = insn.rd(), rs1 = insn.rs1(), shamtd = insn.shamtd());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtd = insn.shamtd();

    todo!();
}