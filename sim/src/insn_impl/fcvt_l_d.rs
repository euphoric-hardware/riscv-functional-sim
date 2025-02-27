use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fcvt_l_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let value = cpu.fload(rs1) as i64 as u64;
    cpu.store(rs1, value);
    Ok(cpu.pc + 4)
}