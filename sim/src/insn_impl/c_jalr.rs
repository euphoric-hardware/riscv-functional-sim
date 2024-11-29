use crate::{bus::Bus, cpu::{self, cr_type, Cpu, Insn}};

pub fn c_jalr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_jalr", c_rs1_n0 = insn.c_rs1_n0());

    let c_rs1_n0 = insn.c_rs1_n0();

    crate::trace_insn(cpu.pc, insn.bits(), "c.jalr", cr_type!(c_rs1_n0, 0));

    cpu.store(1, cpu.pc + 2);
    let new_pc= cpu.load(c_rs1_n0);
    Ok(new_pc)
    
}