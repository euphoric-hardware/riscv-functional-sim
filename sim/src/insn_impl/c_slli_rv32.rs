use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_slli_rv32(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_rs1_n0 = insn.rd_rs1_n0();
    let c_nzuimm6lo = insn.c_nzuimm6lo();

    todo!();
}
