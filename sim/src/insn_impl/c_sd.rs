use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

pub fn c_sd(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rs1_p = insn.rs1_p() + 8;
    let rs2_p = insn.rs2_p() + 8;
    let c_uimm8hi = insn.c_uimm8hi();
    let c_uimm8lo = insn.c_uimm8lo();

    let imm = (c_uimm8lo << 6) | (c_uimm8hi << 3);

    let address = cpu.load(rs1_p).wrapping_add(imm);
    bus.write(address, &cpu.load(rs2_p).to_le_bytes())?;
    Ok(cpu.pc + 2)
}
