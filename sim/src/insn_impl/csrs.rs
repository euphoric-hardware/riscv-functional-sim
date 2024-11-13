use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn csrs(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrs", rs1 = insn.rs1(), csr = insn.csr());

    let rs1 = insn.rs1();
    let csr = insn.csr();

    todo!();
}