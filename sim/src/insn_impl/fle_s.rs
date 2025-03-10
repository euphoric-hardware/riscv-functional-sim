use simple_soft_float::F32;

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fle_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let value = if f32::from_bits(*F32::convert_from_float(&cpu.fload(rs1), None, None).bits())
        <= f32::from_bits(*F32::convert_from_float(&cpu.fload(rs2), None, None).bits())
    {
        1
    } else {
        0
    };
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}
