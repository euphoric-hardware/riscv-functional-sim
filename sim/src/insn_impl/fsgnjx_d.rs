use simple_soft_float::F64;

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fsgnjx_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let sign_bit = (*cpu.fload(rs1).bits() & 0x8000000000000000)
        ^ (*cpu.fload(rs2).bits() & 0x8000000000000000);
    let result = F64::from_bits(*cpu.fload(rs1).bits() & 0x7fffffffffffffff | sign_bit);
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}
