use simple_soft_float::{FPState, StatusFlags};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

use super::insn_raw;

pub fn fnmsub_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rs3 = insn.rs3();
    let rm = insn.rm();

    insn_raw::fnmsub_d_raw::fnmsub_d_raw(cpu, rd, rs1, rs2, rs3, rm)
}