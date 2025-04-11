use simple_soft_float::{FPState, StatusFlags, F32, F64};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmsub_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64, rs3: u64, rm: u64) -> cpu::Result<u64> {
    let mut state = FPState::default();
    let status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let op1 = F32::from_bits(*(cpu.fload(rs1).bits()) as u32);
    let op2 = F32::from_bits(*(cpu.fload(rs2).bits()) as u32);
    let op3 = F32::from_bits(*(cpu.fload(rs3).bits()) as u32);
    // FIXME - update rounding mode (RISC-V -> softfloat)
    let result = op1.fused_mul_add(&op2, &op3.neg(), None, Some(&mut state));
    let result64 = F64::from_bits(0xffffffff00000000 | *result.bits() as u64);
    cpu.fstore(rd, result64);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}