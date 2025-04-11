use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}
};

pub fn jalr_raw(cpu: &mut Cpu, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    
    let offset = Insn::sign_extend(imm_i, 12) as u64;
    let new_pc = cpu.load(rs1).wrapping_add(offset);
    cpu.store(rd, cpu.pc + 4);
    Ok(new_pc)
}