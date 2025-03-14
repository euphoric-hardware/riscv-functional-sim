use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_andi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let c_imm6hi = insn.c_imm6hi();
    let c_imm6lo = insn.c_imm6lo();

    let imm = Insn::sign_extend(c_imm6hi << 5 | c_imm6lo, 6);

    let result = cpu.load(rd_rs1_p) & imm as u64;
    cpu.store(rd_rs1_p, result);

    Ok(cpu.pc + 2)
}
