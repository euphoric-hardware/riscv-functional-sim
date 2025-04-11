use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_jalr_raw(cpu: &mut Cpu, c_rs1_n0: u64) -> cpu::Result<u64> {
    cpu.store(1, cpu.pc + 2);
    let new_pc = cpu.load(c_rs1_n0);
    Ok(new_pc)
}