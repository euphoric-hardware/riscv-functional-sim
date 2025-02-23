use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fcvt_lu_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let value = cpu.fload(rs1) as f32 as u64;
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}