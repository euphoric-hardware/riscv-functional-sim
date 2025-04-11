use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_addi16sp_raw(cpu: &mut Cpu, imm_c_addi16sp: u64) -> cpu::Result<u64> {
    let result = cpu.load(2).wrapping_add(imm_c_addi16sp as u64);
    cpu.store(2, result);
    Ok(cpu.pc + 2)
}