use simple_soft_float::F64;

use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_fld_raw(cpu: &mut Cpu, bus: &mut Bus, rd_p: u64, rs1_p: u64, imm_c_ld: u64) -> cpu::Result<u64> {
    let address = (cpu.load(rs1_p + 8) as u64).wrapping_add(imm_c_ld);

    let mut raw = [0; size_of::<u64>()];
    bus.read(address, &mut raw)?;
    let h = F64::from_bits(u64::from_le_bytes(raw));
    cpu.fstore(rd_p + 8, h);
    Ok(cpu.pc + 2)
}