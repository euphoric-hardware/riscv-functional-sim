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
    let status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    // FIXME - update rounding mode (RISC-V -> softfloat)
    let result = F32::from_bits(*cpu.fload(rs1).bits() as u32).sqrt(None, Some(&mut state));
    let result64 = F64::from_bits(0xffffffff00000000 | *result.bits() as u64);
    cpu.fstore(rd, result64);
    
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}
