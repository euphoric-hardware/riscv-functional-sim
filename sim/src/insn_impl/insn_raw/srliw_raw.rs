use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

#[inline(always)]
pub fn srliw_raw(cpu: &mut Cpu, rd: u64, rs1: u64, shamtw: u64) -> cpu::Result<u64> {
    cpu.store(
        rd,
        Insn::sign_extend((cpu.load(rs1) as u32 >> shamtw) as u64, 32) as u64,
    );
    Ok(cpu.pc + 4)
}
