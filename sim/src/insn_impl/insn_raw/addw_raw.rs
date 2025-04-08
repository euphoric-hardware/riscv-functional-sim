use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn addw_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    cpu.store(
        rd,
        Insn::sign_extend(
            (cpu.load(rs1) as u32).wrapping_add(cpu.load(rs2) as u32) as u64,
            32,
        ) as u64,
    );
    Ok(cpu.pc + 4)
}
