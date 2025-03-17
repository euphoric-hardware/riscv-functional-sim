use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_ebreak(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_ebreak");

    todo!();
}
