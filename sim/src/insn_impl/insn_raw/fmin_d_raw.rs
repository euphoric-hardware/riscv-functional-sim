use simple_soft_float::F64;

use crate::{bus::Bus, cpu::{self, Cpu, Insn}, csrs::Csrs};

pub fn fmin_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let op1 = cpu.fload(rs1);
    let op2 = cpu.fload(rs2);
    let result: F64;
    
    if op1.is_nan() && op2.is_nan() {
        result = F64::quiet_nan();
    } else if op1.is_nan() {
        result = op2;
    } else if op2.is_nan() {
        result = op1;
    } else {
        result = F64::from_bits(f64::min(f64::from_bits(*op1.bits()), f64::from_bits(*op2.bits())).to_bits());
    }

    if op1.is_signaling_nan() || op2.is_signaling_nan() {
        cpu.csrs.store(Csrs::FFLAGS, 16);
    }

    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}