use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_subw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let rs2_p = insn.rs2_p() + 8;

    let result = cpu.load(rd_rs1_p).wrapping_sub(cpu.load(rs2_p)) as u32 as i32 as i64 as u64;

    cpu.store(rd_rs1_p, result);
    Ok(cpu.pc + 2)
}
