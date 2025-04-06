use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn jalr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let offset = Insn::sign_extend(imm12, 12);
    insn_raw::jalr_raw::jalr_raw(cpu, rd, rs1, offset as u64)
}
