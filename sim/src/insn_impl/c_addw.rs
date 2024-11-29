use crate::{bus::Bus, cpu::{self, ca_type, Cpu, Insn}};

pub fn c_addw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_addw", rd_rs1_p = insn.rd_rs1_p(), rs2_p = insn.rs2_p());

    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let rs2_p = insn.rs2_p() + 8;

    crate::trace_insn(cpu.pc, insn.bits(), "c.addw", ca_type!(rd_rs1_p, rs2_p));

    let result = cpu.load(rd_rs1_p).wrapping_add(cpu.load(rs2_p)) as u32 as i32 as i64 as u64;

    cpu.store(rd_rs1_p, result);
    Ok(cpu.pc + 2)
}