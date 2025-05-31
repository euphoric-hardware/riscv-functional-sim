use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

#[inline(always)]
pub fn addiw_raw(cpu: &mut Cpu, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let imm = Insn::sign_extend(imm_i, 12);

    let result = (cpu.load(rs1) as u32).wrapping_add(imm as u32) as u64;
    cpu.store(rd, Insn::sign_extend(result, 32) as u64);
    Ok(cpu.pc + 4)
}
