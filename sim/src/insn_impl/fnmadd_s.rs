use simple_soft_float::{FPState, StatusFlags, F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fnmadd_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rs3 = insn.rs3();
    let rm = insn.rm();

    let mut state = FPState::default();
    let mut status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let op1 = F32::from_bits(*(cpu.fload(rs1).bits()) as u32);
    let op2 = F32::from_bits(*(cpu.fload(rs2).bits()) as u32);
    let op3 = F32::from_bits(*(cpu.fload(rs3).bits()) as u32);
    // FIXME - update rounding mode (RISC-V -> softfloat)
    let result = op1.neg().fused_mul_add(&op2, &op3.neg(), None, Some(&mut state));
    let result64 = F64::from_bits(*result.bits() as u64);

    cpu.fstore(rd, result64);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}
