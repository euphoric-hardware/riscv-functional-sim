use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    csrs::Csrs,
};

pub fn ecall(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("ecall");

    cpu.csrs.store_unchecked(Csrs::MCAUSE, 11);
    Ok(cpu.csrs.load_unchecked(Csrs::MTVEC))
}
