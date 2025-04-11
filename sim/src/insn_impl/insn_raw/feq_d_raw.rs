use simple_soft_float::{FPState, StatusFlags};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn feq_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let mut state = FPState::default();
    let status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
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