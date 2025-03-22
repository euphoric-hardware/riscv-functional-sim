use simple_soft_float::{FPState, StatusFlags, F32, F64};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fdiv_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rm = insn.rm();

    let mut state = FPState::default();
    let status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let op1 = F32::from_bits(*(cpu.fload(rs1).bits()) as u32);
    let op2 = F32::from_bits(*(cpu.fload(rs2).bits()) as u32);
    // FIXME - update rounding mode (RISC-V -> softfloat)
    let result = op1.div(&op2, None, Some(&mut state));
    let result64 = F64::from_bits(0xffffffff00000000 | *result.bits() as u64);

    cpu.fstore(rd, result64);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}