use crate::{
    bus::{Bus, Device},
    cpu::{self, cs_type, Cpu, Insn},
};

pub fn c_sd(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_sd", rs1_p = insn.rs1_p(), rs2_p = insn.rs2_p(), c_uimm8hi = insn.c_uimm8hi(), c_uimm8lo = insn.c_uimm8lo());

    let rs1_p = insn.rs1_p() + 8;
    let rs2_p = insn.rs2_p() + 8;
    let c_uimm8hi = insn.c_uimm8hi();
    let c_uimm8lo = insn.c_uimm8lo();

    let imm = c_uimm8lo << 6 | c_uimm8hi << 3;
    crate::trace_insn(cpu.pc, insn.bits(), "c.sd", cs_type!(rs1_p, rs2_p, imm));

    let address = cpu.load(rs1_p).wrapping_add(imm);
    bus.write(address, &cpu.load(rs2_p).to_le_bytes())?;
    Ok(cpu.pc + 2)
}
