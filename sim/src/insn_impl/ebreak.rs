use crate::{
    bus::Bus,
    cpu::{self, Cpu, Exception, Insn},
};

pub fn ebreak(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    Err(Exception::Breakpoint)
}
