

use crate::{bus::Bus, cpu::{self, Cpu, Insn}, csrs::Csrs};

#[inline(always)]
pub fn fmax_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let op1 = f32::from_bits(cpu.fload(rs1).to_bits() as u32);
    let op2 = f32::from_bits(cpu.fload(rs2).to_bits() as u32);
    let result: f32;
    
    if op1.is_nan() && op2.is_nan() {
        result = f32::NAN;
    } else if f32::is_nan(op1) {
        result = op2;
    } else if f32::is_nan(op2) {
        result = op1;
    } else {
        result = f32::max(op1, op2);
    }

    if Insn::is_signaling_nan_f32(op1) || Insn::is_signaling_nan_f32(op2) {
        cpu.csrs.store(Csrs::FFLAGS, 16);
    }

    cpu.fstore(rd, Insn::f32_to_f64_raw(result));
    Ok(cpu.pc + 4)
}