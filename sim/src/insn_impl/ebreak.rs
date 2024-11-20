use crate::{
    bus::Bus,
    cpu::{self, Cpu, Exception, Insn, InsnType},
};

pub fn ebreak(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn("ecall", InsnType::Privileged);

    Err(Exception::Breakpoint)
}
