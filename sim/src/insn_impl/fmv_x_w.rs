

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

use super::insn_raw;

pub fn fmv_x_w(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    insn_raw::fmv_x_w_raw::fmv_x_w_raw(cpu, rd, rs1)
}