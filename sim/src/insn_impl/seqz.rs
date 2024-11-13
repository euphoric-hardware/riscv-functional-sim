use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn seqz(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("seqz", rd = insn.rd(), rs1 = insn.rs1());

    let rd = insn.rd();
    let rs1 = insn.rs1();

    Ok(cpu.pc + 4)
}