use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fence(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let fm = insn.fm();
    let pred = insn.pred();
    let succ = insn.succ();
    let rs1 = insn.rs1();
    let rd = insn.rd();

    Ok(cpu.pc + 4)
}
