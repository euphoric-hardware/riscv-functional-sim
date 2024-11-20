use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn ebreak(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("ebreak");

    Ok(cpu.pc + 4)
}
