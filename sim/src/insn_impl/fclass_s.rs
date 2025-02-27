use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fclass_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    let value = cpu.fload(rs1) as f32 ;
    let class = f32::classify(value) as u64;
    
    cpu.store(rd, class);
    Ok(cpu.pc + 4)
}