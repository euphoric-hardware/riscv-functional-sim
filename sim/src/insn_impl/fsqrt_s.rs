use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fsqrt_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let value = (cpu.fload(rs1) as f32).sqrt() as f64;
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}