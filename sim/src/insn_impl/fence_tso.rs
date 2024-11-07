use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fence_tso(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("fence_tso", rs1 = insn.rs1(), rd = insn.rd());

    let rs1 = insn.rs1();
    let rd = insn.rd();

    todo!();
}