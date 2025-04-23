use simple_soft_float::{FPState, StatusFlags, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    csrs,
};

use super::insn_raw;

pub fn fadd_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rm = insn.rm();

    insn_raw::fadd_d_raw::fadd_d_raw(cpu, rd, rs1, rs2, rm)
}
