use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn jal(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let jimm20 = insn.jimm20();

    let offset: i64 = Insn::sign_extend(
        ((jimm20 & 0x80000) << 1) | ((jimm20 & 0xff) << 12)
            | ((jimm20 & 0x7fe00) >> 9 << 1)
            | ((jimm20 & 0x100) >> 8 << 11),
        20,
    );

    insn_raw::jal_raw::jal_raw(cpu, rd, offset as u64)
}
