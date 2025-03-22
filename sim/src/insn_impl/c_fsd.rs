use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_fsd(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rs1_p = insn.rs1_p() + 8;
    let rs2_p = insn.rs2_p() + 8;
    let c_uimm8lo = insn.c_uimm8lo();
    let c_uimm8hi = insn.c_uimm8hi();

    let imm = c_uimm8hi << 3 | c_uimm8lo << 6;
    let address = (cpu.load(rs1_p) as u64).wrapping_add(imm as u64);
    let result = *cpu.fload(rs2_p).bits();

    bus.write(address, &result.to_le_bytes());
    Ok(cpu.pc + 2)
}