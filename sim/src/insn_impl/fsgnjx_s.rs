use simple_soft_float::{F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fsgnjx_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let sign = (*cpu.fload(rs2).bits() >> 31) ^(*cpu.fload(rs1).bits() >> 31) ;
    let result = (*cpu.fload(rs1).bits() as u32) & 0x7fffffff | (sign << 31) as u32;
    let result64 = F64::from_bits(0xffffffff00000000 | (result as u64));

    cpu.fstore(rd, result64);
    Ok(cpu.pc + 4)
}
