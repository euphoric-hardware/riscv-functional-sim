use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

use super::insn_raw;

pub fn c_addi4spn(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_p = insn.rd_p();
    let c_nzuimm10 = insn.c_nzuimm10();

    let imm = (c_nzuimm10 & 0xc0) >> 2
        | (c_nzuimm10 & 0x3c) << 4
        | (c_nzuimm10 & 0x02) << 1
        | (c_nzuimm10 & 0x01) << 3;
    insn_raw::c_addi4spn_raw::c_addi4spn_raw(cpu, rd_p, imm)
}
