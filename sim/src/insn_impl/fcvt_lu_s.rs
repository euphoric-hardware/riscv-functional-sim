use simple_soft_float::{FPState, StatusFlags, F32};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, csrs::Csrs,
};

pub fn fcvt_lu_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let mut state = FPState::default();
    let status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let rounding_mode = Insn::softfloat_round_from_riscv_rm(rm);
    let result = F32::from_bits(*cpu.fload(rs1).bits() as u32).to_u64(
        true,
        Some(rounding_mode),
        Some(&mut state),
    );

    if result.is_none() {
        if f32::from_bits(*cpu.fload(rs1).bits() as u32) > u64::MAX as f32 {
            cpu.store(rd, u64::MAX);
        } else if f32::from_bits(*cpu.fload(rs1).bits() as u32) < u64::MIN as f32 {
            cpu.store(rd, u64::MIN);
        } else if F32::from_bits(*cpu.fload(rs1).bits() as u32).is_nan() {
            cpu.store(rd, u64::MAX);
        }
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else {
        cpu.store(rd, result.expect("invalid"));
        Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    }

    Ok(cpu.pc + 4)
}
