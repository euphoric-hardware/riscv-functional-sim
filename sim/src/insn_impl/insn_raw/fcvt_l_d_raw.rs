use simple_soft_float::FPState;

use crate::{bus::Bus, cpu::{self, Cpu, Insn}, csrs::Csrs};

pub fn fcvt_l_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let mut state = FPState::default();
    let status_flags: simple_soft_float::StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let rounding_mode = Insn::softfloat_round_from_riscv_rm(rm);
    let result = cpu
        .fload(rs1)
        .to_i64(true, Some(rounding_mode), Some(&mut state));

    if result.is_none() {
        if f64::from_bits(*cpu.fload(rs1).bits()) > i64::MAX as f64 {
            cpu.store(rd, i64::MAX as u64);
        } else if f64::from_bits(*cpu.fload(rs1).bits()) < i64::MIN as f64 {
            cpu.store(rd, i64::MIN as u64);
        } else if cpu.fload(rs1).is_nan() {
            cpu.store(rd, i64::MAX as u64);
        }
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else {
        cpu.store(rd, result.expect("invalid") as i64 as u64);
        Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    }

    Ok(cpu.pc + 4)
}