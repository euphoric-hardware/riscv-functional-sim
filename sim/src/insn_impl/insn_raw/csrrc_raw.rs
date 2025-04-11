use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn csrrc_raw(cpu: &mut Cpu, rd: u64, rs1: u64, csr: u64) -> cpu::Result<u64> {
    let csr_value = cpu.csrs.load(csr)?;
    let rs1_value = cpu.load(rs1);
    cpu.csrs.store(csr, csr_value & !rs1_value)?;
    cpu.store(rd, csr_value);
    Ok(cpu.pc + 4)
}