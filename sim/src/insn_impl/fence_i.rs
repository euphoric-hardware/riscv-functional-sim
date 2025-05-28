use crate::{cpu::{self, Cpu, Insn}, bus::Bus};
use super::insn_raw;

pub fn fence_i(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let imm12 = insn.imm12();
    let rs1 = insn.rs1();
    let rd = insn.rd();
    insn_raw::fence_i_raw::fence_i_raw(cpu, imm12,rs1,rd)
}