use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_srai_rv32(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_srai_rv32", rd_rs1_p = insn.rd_rs1_p(), c_nzuimm5 = insn.c_nzuimm5());

    let rd_rs1_p = insn.rd_rs1_p();
    let c_nzuimm5 = insn.c_nzuimm5();

    todo!();
}