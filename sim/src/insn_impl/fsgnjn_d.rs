use simple_soft_float::F64;

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fsgnjn_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let result = F64::from_bits(
        (*cpu.fload(rs1).bits() & 0x7fffffffffffffff)
            | !(*cpu.fload(rs1).bits() & 0x8000000000000000),
    );

    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}
