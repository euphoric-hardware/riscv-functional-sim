use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_srai(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let c_nzuimm5 = insn.c_nzuimm5();

    cpu.store(rd_rs1_p, ((cpu.load(rd_rs1_p) as i64) >> c_nzuimm5) as u64);
    Ok(cpu.pc + 2)
}
