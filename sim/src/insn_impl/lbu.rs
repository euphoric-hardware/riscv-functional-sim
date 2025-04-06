use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn lbu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    insn_raw::lbu_raw::lbu_raw(cpu, bus, rd, rs1, imm12)
}
