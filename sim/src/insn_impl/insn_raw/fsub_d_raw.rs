use std::ptr;



use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn fsub_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64, rm: u64) -> cpu::Result<u64> {
    let op1 = unsafe { ptr::read_volatile(&cpu.fload(rs1)) };
    let op2 = unsafe { ptr::read_volatile(&cpu.fload(rs2)) };

    let result = op1 - op2;
    cpu.set_fflags();
    cpu.fstore(rd, result);
    
    Ok(cpu.pc + 4)
}