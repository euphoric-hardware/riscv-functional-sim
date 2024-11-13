use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn jr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("jr", rs1 = insn.rs1());

    let rs1 = insn.rs1();

    Ok(cpu.pc + 4)
}