use crate::{bus::Bus, cpu::{self, ca_type, ci_type, Cpu, Insn}};

pub fn c_add(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_add", rd_rs1_n0 = insn.rd_rs1_n0(), c_rs2_n0 = insn.c_rs2_n0());

    let rd_rs1_n0 = insn.rd_rs1_n0();
    let c_rs2_n0 = insn.c_rs2_n0();

    crate::trace_insn(cpu.pc, insn.bits(), "c.add", ca_type!(rd_rs1_n0, c_rs2_n0));

    let result = cpu.load(rd_rs1_n0).wrapping_add(cpu.load(c_rs2_n0));
    
    cpu.store(rd_rs1_n0, result);
    Ok(cpu.pc + 2)
}