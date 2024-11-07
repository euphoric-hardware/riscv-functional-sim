use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn sraiw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("sraiw", rd = insn.rd(), rs1 = insn.rs1(), shamtw = insn.shamtw());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtw = insn.shamtw();

    todo!();
}