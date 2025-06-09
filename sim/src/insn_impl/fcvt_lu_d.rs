use std::u64;



use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    csrs::Csrs,
};

use super::insn_raw;

pub fn fcvt_lu_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    insn_raw::fcvt_lu_d_raw::fcvt_lu_d_raw(cpu, rd, rs1, rm)
}
