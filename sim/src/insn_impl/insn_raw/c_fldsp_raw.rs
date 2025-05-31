use simple_soft_float::F64;

use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

#[inline(always)]
pub fn c_fldsp_raw(cpu: &mut Cpu, bus: &mut Bus, rd: u64, imm_c_ldsp: u64) -> cpu::Result<u64> {
    let address = cpu.load(2).wrapping_add(imm_c_ldsp);
    let mut raw = [0; size_of::<f64>()];
    bus.read(address, &mut raw)?;
    let h = f64::from_bits(u64::from_le_bytes(raw));
    cpu.fstore(rd, h);
    Ok(cpu.pc + 2)
}