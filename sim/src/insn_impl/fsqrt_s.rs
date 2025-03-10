use simple_soft_float::{FPState, StatusFlags, F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fsqrt_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let mut state = FPState::default();
    let mut status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    // FIXME - update rounding mode (RISC-V -> softfloat)
    let value = F32::convert_from_float(&cpu.fload(rs1), None, None).sqrt(None, Some(&mut state));
    let value64 = F64::convert_from_float(&value, None, None);
    cpu.fstore(rd, value64);
    
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}
