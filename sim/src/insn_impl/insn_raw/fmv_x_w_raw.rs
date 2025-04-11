use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_x_w_raw(cpu: &mut Cpu, rd: u64, rs1: u64) -> cpu::Result<u64> {
    let asf32 = f32::from_bits(*cpu.fload(rs1).bits() as u32);
    let value = u32::from_le_bytes(asf32.to_le_bytes()) as i32 as u64;
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}