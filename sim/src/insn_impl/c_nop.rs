use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_nop(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_nop", c_nzimm6hi = insn.c_nzimm6hi(), c_nzimm6lo = insn.c_nzimm6lo());

    let c_nzimm6hi = insn.c_nzimm6hi();
    let c_nzimm6lo = insn.c_nzimm6lo();

    Ok(cpu.pc + 2)
}