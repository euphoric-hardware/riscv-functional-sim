use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fsqrt_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    todo!();
}