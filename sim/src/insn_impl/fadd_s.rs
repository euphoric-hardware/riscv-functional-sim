use simple_soft_float::{F32, F64, FPState, StatusFlags};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fadd_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rm = insn.rm();
    
    let mut state = FPState::default();
    let mut status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let op1 = F32::convert_from_float(&cpu.fload(rs1), None, None);
    let op2 = F32::convert_from_float(&cpu.fload(rs2), None, None);
    // FIXME - update rounding mode (RISC-V -> softfloat)
    let result = op1.add(&op2, None, Some(&mut state));
    let result64 = F64::convert_from_float(&result, None, None);

    cpu.fstore(rd, result64);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}