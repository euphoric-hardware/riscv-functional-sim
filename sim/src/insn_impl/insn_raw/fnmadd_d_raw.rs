use std::ptr;

use simple_soft_float::{FPState, StatusFlags};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fnmadd_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64, rs3: u64, rm: u64) -> cpu::Result<u64> {
    let op1 = unsafe { ptr::read_volatile(&cpu.fload(rs1)) };
    let op2 = unsafe { ptr::read_volatile(&cpu.fload(rs2)) };
    let op3 = unsafe { ptr::read_volatile(&cpu.fload(rs3)) };
    let result = -(op1 * op2) - op3;
    
    cpu.set_fflags();
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}