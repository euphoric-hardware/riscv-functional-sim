use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_srai_rv32(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_rs1_p = insn.rd_rs1_p();
    let c_nzuimm5 = insn.c_nzuimm5();

    todo!();
}
