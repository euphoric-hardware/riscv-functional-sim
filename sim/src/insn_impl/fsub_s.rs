

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn fsub_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rm = insn.rm();

    insn_raw::fsub_s_raw::fsub_s_raw(cpu, rd, rs1, rs2, rm)
}
