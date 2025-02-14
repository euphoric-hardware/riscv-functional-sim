use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_x_w(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    let bytes = f32::to_le_bytes(cpu.fload(rs1) as f32);
    cpu.store(rd, u32::from_le_bytes(bytes) as u64);
    Ok(cpu.pc + 4)
}