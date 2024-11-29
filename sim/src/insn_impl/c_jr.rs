use crate::{bus::Bus, cpu::{self, cr_type, Cpu, Insn}};

pub fn c_jr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rs1_n0 = insn.rs1_n0();

    crate::trace_insn(cpu.pc, insn.bits(), "c.jr", cr_type!(rs1_n0, 0));
    
    let new_pc= cpu.load(rs1_n0);
    Ok(new_pc)
}