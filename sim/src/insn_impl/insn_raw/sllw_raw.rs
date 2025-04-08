use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sllw_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let result = Insn::sign_extend(
        cpu.load(rs1).wrapping_shl((cpu.load(rs2) & 0x1f) as u32) as u32 as u64,
        32,
    ) as u64;
    cpu.store(rd, result);
    Ok(cpu.pc + 4)
}
