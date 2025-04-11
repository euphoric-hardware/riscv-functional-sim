use crate::{bus::Bus, cpu::{self, Cpu, Insn}, csrs::Csrs};

pub fn mret_raw(cpu: &mut Cpu) -> cpu::Result<u64> {
    Ok(cpu.csrs.load_unchecked(Csrs::MEPC))
}