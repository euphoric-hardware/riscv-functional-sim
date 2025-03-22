use simple_soft_float::F64;

use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_fld(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_p = insn.rd_p() + 8;
    let rs1_p = insn.rs1_p() + 8;
    let c_uimm8lo = insn.c_uimm8lo();
    let c_uimm8hi = insn.c_uimm8hi();

    let imm = c_uimm8lo << 6 | c_uimm8hi << 3;
    let address = (cpu.load(rs1_p) as u64).wrapping_add(imm as u64);

    let mut raw = [0; size_of::<u64>()];
    bus.read(address, &mut raw)?;
    let h = F64::from_bits(u64::from_le_bytes(raw));
    cpu.fstore(rd_p, h);
    Ok(cpu.pc + 2)
}