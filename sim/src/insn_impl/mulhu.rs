use crate::{bus::Bus, cpu::{self, Cpu, Insn}};

pub fn mulhu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    

    let result = (cpu.load(rs1) as u128 * cpu.load(rs2) as u128) >> 64;
    cpu.store(rd, result as u64);

    Ok(cpu.pc + 4)
}