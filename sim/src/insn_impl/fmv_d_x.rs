

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

use super::insn_raw;

pub fn fmv_d_x(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    insn_raw::fmv_d_x_raw::fmv_d_x_raw(cpu, rd, rs1)
}