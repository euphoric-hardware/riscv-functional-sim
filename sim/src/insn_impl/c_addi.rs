use crate::{bus::Bus, cpu::{self, ci_type, Cpu, Insn}};

pub fn c_addi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_addi", rd_rs1_n0 = insn.rd_rs1_n0(), c_nzimm6lo = insn.c_nzimm6lo(), c_nzimm6hi = insn.c_nzimm6hi());

    let rd_rs1_n0 = insn.rd_rs1_n0();
    let c_nzimm6lo = insn.c_nzimm6lo();
    let c_nzimm6hi = insn.c_nzimm6hi();

    let imm = Insn::sign_extend(c_nzimm6hi << 5 | c_nzimm6lo, 6);
    

    crate::trace_insn(cpu.pc, insn.bits(), "c.addi", ci_type!(rd_rs1_n0, imm));

    let result = cpu.load(rd_rs1_n0).wrapping_add(imm as u64);
    cpu.store(rd_rs1_n0, result);
    Ok(cpu.pc + 2)
}