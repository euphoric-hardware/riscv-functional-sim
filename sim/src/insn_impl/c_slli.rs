use crate::{bus::Bus, cpu::{self, ci_type, Cpu, Insn}};

pub fn c_slli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_slli", rd_rs1_n0 = insn.rd_rs1_n0(), c_nzuimm6lo = insn.c_nzuimm6lo());

    let rd_rs1_n0 = insn.rd_rs1_n0();
    let c_nzuimm6lo = insn.c_nzuimm6lo(); // assume high bit is 1 for rv32 and rv64

    let imm = 0 << 5 | c_nzuimm6lo;
    let result = cpu.load(rd_rs1_n0).wrapping_shl(imm as u32);
    crate::trace_insn(cpu.pc, insn.bits(), "c.slli", ci_type!(rd_rs1_n0, c_nzuimm6lo));
    
    cpu.store(rd_rs1_n0, result);
    Ok(cpu.pc + 2)
}