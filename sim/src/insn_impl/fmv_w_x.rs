use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_w_x(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    let bytes = (cpu.load(rs1) as f32 as u32).to_le_bytes();
    cpu.fstore(rd, f32::from_le_bytes(bytes) as f64);
    Ok(cpu.pc + 4)
}