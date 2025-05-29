use std::ptr;

use simple_soft_float::{FPState, StatusFlags};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn feq_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {

    let op1 = unsafe { ptr::read_volatile(&cpu.fload(rs1)) };
    let op2 = unsafe { ptr::read_volatile(&cpu.fload(rs2)) };

    let value = if op1 == op2 {
        1
    } else {
        0
    };
    cpu.set_fflags();
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}