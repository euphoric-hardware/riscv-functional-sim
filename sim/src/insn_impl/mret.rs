use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    csrs::Csrs,
};

pub fn mret(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("mret");

    Ok(cpu.csrs.load_unchecked(Csrs::MEPC))
}
