use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_slli_rv32(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_slli_rv32", rd_rs1_n0 = insn.rd_rs1_n0(), c_nzuimm6lo = insn.c_nzuimm6lo());

    let rd_rs1_n0 = insn.rd_rs1_n0();
    let c_nzuimm6lo = insn.c_nzuimm6lo();

    todo!();
}