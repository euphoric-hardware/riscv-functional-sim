use crate::{
    bus::Bus,
    cpu::{self, Cpu, Exception, Insn},
    csrs::Csrs,
};

pub fn ecall(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("ecall");

    Err(Exception::EnvironmentCallFromMMode)
}
