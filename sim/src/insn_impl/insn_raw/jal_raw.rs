use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}
};

pub fn jal_raw(cpu: &mut Cpu, rd: u64, imm_j: u64) -> cpu::Result<u64> {
    let offset = imm_j;

    cpu.store(rd, cpu.pc + 4);
    Ok(cpu.pc.wrapping_add(offset))
}
