use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_srli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let c_nzuimm6lo = insn.c_nzuimm6lo();
    let c_nzuimm6hi = insn.c_nzuimm6hi();
    
    let imm =  (c_nzuimm6hi << 5) | c_nzuimm6lo;
    let result = cpu.load(rd_rs1_p).wrapping_shr(imm as u32);
    cpu.store(rd_rs1_p, result);
    Ok(cpu.pc + 2)
}
