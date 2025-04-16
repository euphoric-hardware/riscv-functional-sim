use simple_soft_float::{F32, F64};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

use super::insn_raw;

pub fn fcvt_s_w(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    insn_raw::fcvt_s_w_raw::fcvt_s_w_raw(cpu, rd, rs1, rm)
}