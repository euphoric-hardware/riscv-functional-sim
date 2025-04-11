use simple_soft_float::{FPState, StatusFlags, F64};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fcvt_d_w_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let mut state = FPState::default();
    let status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let result = F64::from_i32(cpu.load(rs1) as i32, Some(Insn::softfloat_round_from_riscv_rm(rm)), Some(&mut state));
    
    cpu.fstore(rd, result);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}