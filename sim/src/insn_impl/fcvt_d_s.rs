use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fcvt_d_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("fcvt_d_s", rd = insn.rd(), rs1 = insn.rs1(), rm = insn.rm());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    todo!();
}