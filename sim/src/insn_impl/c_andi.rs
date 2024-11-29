use crate::{bus::Bus, cpu::{self, cb_type, Cpu, Insn}, trace_insn};

pub fn c_andi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_andi", rd_rs1_p = insn.rd_rs1_p(), c_imm6hi = insn.c_imm6hi(), c_imm6lo = insn.c_imm6lo());

    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let c_imm6hi = insn.c_imm6hi();
    let c_imm6lo = insn.c_imm6lo();

    let imm = Insn::sign_extend(c_imm6hi << 5 | c_imm6lo, 6);

    crate::trace_insn(cpu.pc, insn.bits(), "c.andi", cb_type!(rd_rs1_p, imm));

    let result = cpu.load(rd_rs1_p) & imm as u64;
    cpu.store(rd_rs1_p, result);
    
    Ok(cpu.pc + 2)
    
}