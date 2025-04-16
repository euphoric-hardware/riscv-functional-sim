use std::ptr;

use simple_soft_float::{FPState, StatusFlags, F32, F64};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fdiv_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64, rm: u64) -> cpu::Result<u64> {
    let op1 = unsafe { ptr::read_volatile(&f32::from_bits(cpu.fload(rs1) as u32)) };
    let op2 = unsafe { ptr::read_volatile(&f32::from_bits(cpu.fload(rs2) as u32)) };

    let result = op1 / op2;
    cpu.set_fflags();
    cpu.fstore(rd, Insn::f32_to_f64_raw(result));
    
    Ok(cpu.pc + 4)
}