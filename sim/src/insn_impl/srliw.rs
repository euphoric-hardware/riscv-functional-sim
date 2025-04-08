use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn srliw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtw = insn.shamtw();

    insn_raw::srliw_raw::srliw_raw(cpu, rd, rs1, shamtw)
}
