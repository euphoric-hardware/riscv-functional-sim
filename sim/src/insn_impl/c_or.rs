use crate::{bus::Bus, cpu::{self, Cpu, Insn}};

pub fn c_or(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_or", rd_rs1_p = insn.rd_rs1_p(), rs2_p = insn.rs2_p());

    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let rs2_p = insn.rs2_p() + 8;

    
    let result = cpu.load(rd_rs1_p) | cpu.load(rs2_p);
    cpu.store(rd_rs1_p, result);

    Ok(cpu.pc + 2)
}