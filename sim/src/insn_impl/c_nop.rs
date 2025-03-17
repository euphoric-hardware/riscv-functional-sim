use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_nop(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let c_nzimm6hi = insn.c_nzimm6hi();
    let c_nzimm6lo = insn.c_nzimm6lo();

    Ok(cpu.pc + 2)
}
