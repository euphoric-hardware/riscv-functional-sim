use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn csrci(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrci", csr = insn.csr(), zimm = insn.zimm());

    let csr = insn.csr();
    let zimm = insn.zimm();

    Ok(cpu.pc + 4)
}