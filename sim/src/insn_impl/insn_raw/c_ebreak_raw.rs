use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn c_ebreak_raw(cpu: &mut Cpu) -> cpu::Result<u64> {
    todo!();
}