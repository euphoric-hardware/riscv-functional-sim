use simple_soft_float::{FPState, StatusFlags};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fsqrt_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let mut state = FPState::default();
    let status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let value = (cpu.fload(rs1)).sqrt(None, Some(&mut state));
    
    cpu.fstore(rd, value);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}