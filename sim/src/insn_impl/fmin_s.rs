use simple_soft_float::{F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    csrs::Csrs,
};

pub fn fmin_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let op1 = f32::from_bits(*cpu.fload(rs1).bits() as u32);
    let op2 = f32::from_bits(*cpu.fload(rs2).bits() as u32);
    let mut result: F64;

    if (F32::from_bits(op1.to_bits()).is_nan() && F32::from_bits(op2.to_bits()).is_nan()) {
        result = F64::quiet_nan();
    } else if (F32::from_bits(op1.to_bits()).is_nan()) {
        result = F64::from_bits(op2.to_bits() as u64);
    } else if (F32::from_bits(op2.to_bits()).is_nan()) {
        result = F64::from_bits(op2.to_bits() as u64);
    } else {
        result = F64::from_bits(f32::min(op1, op2).to_bits() as u64);
    }

    if (F32::from_bits(op1.to_bits()).is_signaling_nan()
        || F32::from_bits(op2.to_bits()).is_signaling_nan())
    {
        cpu.csrs.store(Csrs::FFLAGS, 16);
    }

    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}
