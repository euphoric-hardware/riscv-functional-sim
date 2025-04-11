use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}
};

pub fn lui_raw(cpu: &mut Cpu, rd: u64, imm_u: u64) -> cpu::Result<u64> {
    cpu.store(rd, imm_u);
    Ok(cpu.pc + 4)
}