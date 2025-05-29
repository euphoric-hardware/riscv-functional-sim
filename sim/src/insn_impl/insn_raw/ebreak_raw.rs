use crate::{bus::Bus, cpu::{self, Cpu, Exception, Insn}};

#[inline(always)]
pub fn ebreak_raw(cpu: &mut Cpu) -> cpu::Result<u64> {
    Err(Exception::Breakpoint)
}