use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_x_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    let result = u64::from_le_bytes(f64::from_bits(*cpu.fload(rs1).bits()).to_le_bytes());
    cpu.store(rd, result);
    Ok(cpu.pc + 4)
    
}