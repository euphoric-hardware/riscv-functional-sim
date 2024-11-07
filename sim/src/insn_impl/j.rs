use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn j(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("j", jimm20 = insn.jimm20());

    let jimm20 = insn.jimm20();

    todo!();
}