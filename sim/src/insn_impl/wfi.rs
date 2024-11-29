use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn, InsnType},
};

pub fn wfi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn("wfi", InsnType::Privileged);

    Ok(cpu.pc)
}
