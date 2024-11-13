use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn csrrs(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrrs", rd = insn.rd(), rs1 = insn.rs1(), csr = insn.csr());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let csr = insn.csr();

    todo!();
}