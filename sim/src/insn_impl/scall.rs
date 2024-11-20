use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn scall(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("scall");

    Ok(cpu.pc + 4)
}
