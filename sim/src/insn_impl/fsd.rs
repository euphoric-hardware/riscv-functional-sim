use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fsd(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let imm12hi = insn.imm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let imm12lo = insn.imm12lo();

    todo!();
}