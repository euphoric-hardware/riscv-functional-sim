use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fcvt_d_w(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let value = cpu.load(rs1) as i32 as f64;
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}