use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}
};

pub fn slti_raw(cpu: &mut Cpu, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let signed_imm = Insn::sign_extend(imm_i, 12);
    let result = if (cpu.load(rs1) as i64) < signed_imm {
        1
    } else {
        0
    };
    cpu.store(rd, result);
    Ok(cpu.pc + 4)
}
