use simple_soft_float::{FPState, RoundingMode, StatusFlags, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fcvt_w_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let mut state = FPState::default();
    let mut status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    // FIXME - rounding mode
    let result =
        F64::to_i32(&cpu.fload(rs1), true, None, Some(&mut state)).expect("invalid conversion") as i64 as u64;
    cpu.store(rd, result);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}
