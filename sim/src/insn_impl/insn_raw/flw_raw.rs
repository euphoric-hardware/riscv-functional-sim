use simple_soft_float::F64;

use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

#[inline(always)]
pub fn flw_raw(cpu: &mut Cpu, bus: &mut Bus, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(imm_i as u64, 12);
    let address = (cpu.load(rs1) as u64).wrapping_add(offset as u64);

    let mut raw = [0; size_of::<u32>()];
    bus.read(address, &mut raw)?;
    let h = u32::from_le_bytes(raw) as u64;
    let h64 = f64::from_bits((u64::MAX & 0xffffffff00000000) | h);

    cpu.fstore(rd, h64);
    Ok(cpu.pc + 4)
}