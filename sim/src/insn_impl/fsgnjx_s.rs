use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fsgnjx_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let sign_bit = ((f32::to_bits(cpu.fload(rs1) as f32) & 0x80000000) ^ (f32::to_bits(cpu.fload(rs2) as f32) & 0x80000000));
    let value = f32::from_bits(f32::to_bits(cpu.fload(rs1) as f32) & 0x7fffffff | sign_bit) as f64;
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}