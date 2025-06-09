

use crate::{bus::Bus, cpu::{self, Cpu, Insn}, csrs::Csrs};

#[inline(always)]
pub fn fmin_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let op1 = cpu.fload(rs1);
    let op2 = cpu.fload(rs2);
    let result: f64;
    
    if op1.is_nan() && op2.is_nan() {
        result = f64::NAN;
    } else if f64::is_nan(op1) {
        result = op2;
    } else if f64::is_nan(op2) {
        result = op1;
    } else {
        result = f64::min(op1, op2);
    }

    if Insn::is_signaling_nan_f64(op1) || Insn::is_signaling_nan_f64(op2) {
        cpu.csrs.store(Csrs::FFLAGS, 16);
    }

    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}