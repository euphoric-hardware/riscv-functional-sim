use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn csrrwi_raw(cpu: &mut Cpu, rd: u64, csr: u64, zimm: u64) -> cpu::Result<u64> {
    let csr_value = cpu.csrs.load(csr)?;
    cpu.csrs.store(csr, zimm)?;
    cpu.store(rd, csr_value);
    Ok(cpu.pc + 4)
}