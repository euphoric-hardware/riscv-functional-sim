

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

use super::insn_raw;

pub fn fcvt_s_lu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    insn_raw::fcvt_s_lu_raw::fcvt_s_lu_raw(cpu, rd, rs1, rm)
}