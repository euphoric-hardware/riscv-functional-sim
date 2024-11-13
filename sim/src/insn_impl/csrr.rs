use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn csrr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrr", rd = insn.rd(), csr = insn.csr());

    let rd = insn.rd();
    let csr = insn.csr();

    Ok(cpu.pc + 4)
}