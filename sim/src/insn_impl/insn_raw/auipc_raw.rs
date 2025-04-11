use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}
};

pub fn auipc_raw(cpu: &mut Cpu, rd: u64, imm_u: u64) -> cpu::Result<u64> {
    let value = cpu.pc.wrapping_add(imm_u);
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}