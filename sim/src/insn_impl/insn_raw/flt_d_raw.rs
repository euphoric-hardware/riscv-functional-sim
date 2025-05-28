use std::{cmp::Ordering, ptr};

use simple_soft_float::{FPState, StatusFlags};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn flt_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let mut op1 = unsafe { ptr::read_volatile(&cpu.fload(rs1)) };
    let mut op2 = unsafe { ptr::read_volatile(&cpu.fload(rs2)) };

    // ugly workaround for nan comparison. need to check behavior for x86
    #[cfg(target_arch = "aarch64")]
    {
        // set any nan to signaling so that we get inexact flag checking
        if f64::is_nan(op1) {
            op1 = unsafe { ptr::read_volatile(&f64::from_bits(0x7FF7FFFFFFFFFFFF)) };
        }

        if f64::is_nan(op2) {
            op2 = unsafe { ptr::read_volatile(&f64::from_bits(0x7FF7FFFFFFFFFFFF)) };
        }
    }

    let value = if op1 < op2 { 1 } else { 0 };

    cpu.set_fflags();
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}