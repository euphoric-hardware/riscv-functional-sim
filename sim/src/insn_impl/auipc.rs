use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn auipc(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let imm20 = insn.imm20();

    insn_raw::auipc_raw::auipc_raw(cpu, rd, Insn::sign_extend(imm20 << 12, 32) as u64)
}
