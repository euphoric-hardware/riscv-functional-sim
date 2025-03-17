use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn zext_b(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    Ok(cpu.pc + 4)
}
