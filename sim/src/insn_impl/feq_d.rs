use std::prelude::rust_2024;

use simple_soft_float::{FPState, StatusFlags};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn feq_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let mut state = FPState::default();
    let mut status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let op1 = cpu.fload(rs1);
    let op2 = cpu.fload(rs2);

    let value= if op1.compare_quiet(&op2, Some(&mut state)) == Some(std::cmp::Ordering::Equal) {
        1
    } else {
        0
    };
    cpu.store(rd, value);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}