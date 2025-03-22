use simple_soft_float::{F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fsgnj_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let sign = ((*cpu.fload(rs2).bits() as u32) & 0x80000000) as u32;
    let result = (*cpu.fload(rs1).bits() as u32) | sign;
    let result64 = F64::from_bits(0xffffffff00000000 | (result as u64));
    
    cpu.fstore(rd, result64);
    Ok(cpu.pc + 4)
}
