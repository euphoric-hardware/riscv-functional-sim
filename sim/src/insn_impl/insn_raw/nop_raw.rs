use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}
};

pub fn nop_raw(cpu: &mut Cpu) -> cpu::Result<u64> {
    Ok(cpu.pc + 4)
}