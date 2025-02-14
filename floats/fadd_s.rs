use crate::{bus::Bus, cpu::{self, r_type, Cpu, Insn}};

pub fn fadd_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rm = insn.rm();

    let value = cpu.fload(rs1) + cpu.fload(rs2);
    cpu.fstore(rd,  value);
    
    Ok(cpu.pc + 4)
}