use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn bne(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let bimm12hi = insn.bimm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let bimm12lo = insn.bimm12lo();

    let offset = Insn::sign_extend(
        ((bimm12hi & 0x40) << 6)
            | ((bimm12lo & 0x01) << 11)
            | ((bimm12hi & 0x3F) << 5)
            | (bimm12lo & 0x1E),
        13,
    );

    insn_raw::bne_raw::bne_raw(cpu, rs1, rs2, offset as u64)
}
