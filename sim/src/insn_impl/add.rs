use crate::{cpu::{self, Cpu, Insn}, bus::Bus};
use super::insn_raw;

pub fn add(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    insn_raw::add_raw::add_raw(cpu, rd,rs1,rs2)
}