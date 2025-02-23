use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fnmsub_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("fnmsub_d", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2(), rs3 = insn.rs3(), rm = insn.rm());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rs3 = insn.rs3();
    let rm = insn.rm();

    todo!();
}