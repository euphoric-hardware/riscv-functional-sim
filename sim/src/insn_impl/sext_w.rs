use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sext_w(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("sext_w", rd = insn.rd(), rs1 = insn.rs1());

    let rd = insn.rd();
    let rs1 = insn.rs1();

    Ok(cpu.pc + 4)
}
