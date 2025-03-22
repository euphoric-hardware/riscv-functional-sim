use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_jalr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let c_rs1_n0 = insn.c_rs1_n0();
    cpu.store(1, cpu.pc + 2);
    let new_pc = cpu.load(c_rs1_n0);
    Ok(new_pc)
}
