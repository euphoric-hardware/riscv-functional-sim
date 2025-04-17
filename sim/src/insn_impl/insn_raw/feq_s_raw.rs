use std::{cmp::Ordering, ptr};

use simple_soft_float::{FPState, StatusFlags, F32};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn feq_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let op1 = unsafe { ptr::read_volatile(&f32::from_bits(cpu.fload(rs1).to_bits() as u32)) };
    let op2 = unsafe { ptr::read_volatile(&f32::from_bits(cpu.fload(rs2).to_bits() as u32)) };

    let value= if op1 == op2 {
        1
    } else {
        0
    };
    
    cpu.set_fflags();
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}