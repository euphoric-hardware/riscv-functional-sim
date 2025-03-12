use simple_soft_float::F32;

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_x_w(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    let asf32 = f32::from_bits(*cpu.fload(rs1).bits() as u32);
    let value = u32::from_le_bytes(asf32.to_le_bytes()) as i32 as u64;
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}