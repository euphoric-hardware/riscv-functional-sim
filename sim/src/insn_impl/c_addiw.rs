use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_addiw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_addiw", rd_rs1_n0 = insn.rd_rs1_n0(), c_imm6lo = insn.c_imm6lo(), c_imm6hi = insn.c_imm6hi());

    let rd_rs1_n0 = insn.rd_rs1_n0();
    let c_imm6lo = insn.c_imm6lo();
    let c_imm6hi = insn.c_imm6hi();

    let imm = Insn::sign_extend(c_imm6hi << 5 | c_imm6lo, 6);

    let result = cpu.load(rd_rs1_n0).wrapping_add(imm as u64) as u32 as i32 as i64 as u64;
    cpu.store(rd_rs1_n0, result);
    Ok(cpu.pc + 2)
}
