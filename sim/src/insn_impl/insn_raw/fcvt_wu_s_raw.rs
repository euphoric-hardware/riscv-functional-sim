use simple_soft_float::{FPState, StatusFlags, F32};

use crate::{bus::Bus, cpu::{self, Cpu, Insn}, csrs::Csrs};

pub fn fcvt_wu_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let mut state = FPState::default();
    let status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let rounding_mode = Insn::softfloat_round_from_riscv_rm(rm);
    let result: Option<u32> = F32::from_bits(*cpu.fload(rs1).bits() as u32).to_u32(
        true,
        Some(rounding_mode),
        Some(&mut state),
    );

    if result.is_none() {
        if f32::from_bits(*cpu.fload(rs1).bits() as u32) > u32::MAX as f32 {
            cpu.store(rd, (u32::MAX) as i32 as i64 as u64);
        } else if f32::from_bits(*cpu.fload(rs1).bits() as u32) < u32::MIN as f32 {
            cpu.store(rd, (u32::MIN) as i32 as i64 as u64);
        } else if F32::from_bits(*cpu.fload(rs1).bits() as u32).is_nan() {
            cpu.store(rd, u32::MAX as i32 as i64 as u64);
        }
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else {
        cpu.store(rd, result.expect("invalid") as i32 as i64 as u64);
        Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    }

    Ok(cpu.pc + 4)
}