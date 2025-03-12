use std::cmp::Ordering;

use simple_soft_float::{FPState, StatusFlags, F32};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fle_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let mut state = FPState::default();
    let mut status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let op1 = cpu.fload(rs1);
    let op2 = cpu.fload(rs2);

    let value = if matches!(
        op1.compare_signaling(&op2, Some(&mut state)),
        Some(Ordering::Less) |
        Some(Ordering::Equal)
    ) {
        1
    } else {
        0
    };
    
    cpu.store(rd, value);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}
