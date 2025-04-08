use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sraiw_raw(cpu: &mut Cpu, rd: u64, rs1: u64, shamtw: u64) -> cpu::Result<u64> {
    cpu.store(rd, ((cpu.load(rs1) as i32) >> shamtw) as i64 as u64);
    Ok(cpu.pc + 4)
}
