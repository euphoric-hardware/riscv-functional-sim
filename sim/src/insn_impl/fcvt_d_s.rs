

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn fcvt_d_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    insn_raw::fcvt_d_s_raw::fcvt_d_s_raw(cpu, rd, rs1, rm)
}
