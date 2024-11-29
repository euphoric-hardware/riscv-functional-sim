use crate::{bus::Bus, cpu::{self, cb_type, Cpu, Insn}};

pub fn c_srli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_srli", rd_rs1_p = insn.rd_rs1_p(), c_nzuimm5 = insn.c_nzuimm5());

    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let c_nzuimm5 = insn.c_nzuimm5();

    crate::trace_insn(cpu.pc, insn.bits(), "c.srli", cb_type!(rd_rs1_p, c_nzuimm5));

    let result = cpu.load(rd_rs1_p).wrapping_shr(c_nzuimm5 as u32);
    cpu.store(rd_rs1_p, result);
    Ok(cpu.pc + 2)
}