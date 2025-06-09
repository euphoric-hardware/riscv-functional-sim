use std::prelude::rust_2024;



use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

use super::insn_raw;

pub fn feq_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    insn_raw::feq_d_raw::feq_d_raw(cpu, rd, rs1, rs2)
}