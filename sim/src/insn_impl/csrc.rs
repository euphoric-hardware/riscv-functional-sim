use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn csrc(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrc", rs1 = insn.rs1(), csr = insn.csr());

    let rs1 = insn.rs1();
    let csr = insn.csr();

    Ok(cpu.pc + 4)
}