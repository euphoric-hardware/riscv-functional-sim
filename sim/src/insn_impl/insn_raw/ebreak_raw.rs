use crate::{bus::Bus, cpu::{self, Cpu, Exception, Insn}};

pub fn ebreak_raw(cpu: &mut Cpu) -> cpu::Result<u64> {
    Err(Exception::Breakpoint)
}