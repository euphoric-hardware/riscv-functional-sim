use simple_soft_float::{FPState, StatusFlags};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fsub_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rm = insn.rm();

    let mut state = FPState::default();
    let mut status_flags: StatusFlags = Insn::softfloat_flags_from_riscv_flags(cpu);
    state.status_flags = status_flags;

    let op1 = cpu.fload(rs1);
    let op2 = cpu.fload(rs2);
    
    // FIXME - update rounding mode (RISC-V -> softfloat)
    let result = op1.sub(&op2, None, Some(&mut state));

    cpu.fstore(rd, result);
    Insn::riscv_flags_from_softfloat_flags(cpu, state.status_flags);
    Ok(cpu.pc + 4)
}
