use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fence_tso(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("fence_tso", rs1 = insn.rs1(), rd = insn.rd());

    let rs1 = insn.rs1();
    let rd = insn.rd();

    Ok(cpu.pc + 4)
}
