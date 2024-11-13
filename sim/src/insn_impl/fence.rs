use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fence(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("fence", fm = insn.fm(), pred = insn.pred(), succ = insn.succ(), rs1 = insn.rs1(), rd = insn.rd());

    let fm = insn.fm();
    let pred = insn.pred();
    let succ = insn.succ();
    let rs1 = insn.rs1();
    let rd = insn.rd();

    Ok(cpu.pc + 4)
}