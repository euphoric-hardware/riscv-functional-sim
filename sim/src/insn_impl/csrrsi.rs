use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn csrrsi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrrsi", rd = insn.rd(), csr = insn.csr(), zimm = insn.zimm());

    let rd = insn.rd();
    let csr = insn.csr();
    let zimm = insn.zimm();

    Ok(cpu.pc + 4)
}