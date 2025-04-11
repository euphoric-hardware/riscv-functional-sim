use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_bnez_raw(cpu: &mut Cpu, rs1_p: u64, imm_c_b: u64) -> cpu::Result<u64> {
    let mut new_pc = cpu.pc + 2;
    if cpu.load(rs1_p + 8) != 0 {
        new_pc = cpu.pc.wrapping_add(Insn::sign_extend(imm_c_b, 9) as u64);
    }
    Ok(new_pc)
}