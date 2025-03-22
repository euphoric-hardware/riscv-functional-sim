use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_slli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_rs1_n0 = insn.rd_rs1_n0();
    let c_nzuimm6lo = insn.c_nzuimm6lo();
    let c_nzuimm6hi = insn.c_nzuimm6hi();

    let imm =  c_nzuimm6hi << 5 | c_nzuimm6lo;
    let result = cpu.load(rd_rs1_n0).wrapping_shl(imm as u32);
    cpu.store(rd_rs1_n0, result);
    Ok(cpu.pc + 2)
}
