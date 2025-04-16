use simple_soft_float::F32;

use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn fsw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let imm12hi = insn.imm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let imm12lo = insn.imm12lo();

    let offset = Insn::sign_extend((imm12hi << 5 | imm12lo) as u64, 12);

    insn_raw::fsw_raw::fsw_raw(cpu, bus, rs1, rs2, offset as u64)
}
