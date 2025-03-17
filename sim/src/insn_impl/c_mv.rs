use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_mv(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_n0 = insn.rd_n0();
    let c_rs2_n0 = insn.c_rs2_n0();

    cpu.store(rd_n0, cpu.load(c_rs2_n0));

    Ok(cpu.pc + 2)
}
