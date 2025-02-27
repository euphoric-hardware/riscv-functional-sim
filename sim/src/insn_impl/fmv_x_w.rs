use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_x_w(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    let asf32 = cpu.fload(rs1) as f32;
    println!("asf32 = {}", asf32);
    let value = u32::from_le_bytes(f32::to_le_bytes(cpu.fload(rs1) as f32)) as i32 as u64;
    println!("value = {:#16x}", value);
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}