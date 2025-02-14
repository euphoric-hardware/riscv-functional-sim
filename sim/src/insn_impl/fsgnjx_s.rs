use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fsgnjx_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let sign_bit = ((cpu.fload(rs2) < 0.0)  ^ (cpu.fload(rs1) < 0.0)) as u32;
    let value = (-1.0 as u64).pow(sign_bit) as f64 * cpu.fload(rs1);
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}