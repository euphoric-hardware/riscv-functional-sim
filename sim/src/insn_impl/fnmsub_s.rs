use simple_soft_float::{FPState, StatusFlags, F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fnmsub_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rs3 = insn.rs3();
    let rm = insn.rm();

    let mut state = FPState::default();
    let mut status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let op1 = F32::convert_from_float(&cpu.fload(rs1), None, None);
    let op2 = F32::convert_from_float(&cpu.fload(rs2), None, None);
    let op3 = F32::convert_from_float(&cpu.fload(rs3), None, None);
    // FIXME - update rounding mode (RISC-V -> softfloat)
    let result = op1.neg().fused_mul_add(&op2, &op3, None, Some(&mut state));
    let result64 = F64::convert_from_float(&result, None, None);

    cpu.fstore(rd, result64);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}
