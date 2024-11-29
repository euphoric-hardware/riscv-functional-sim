use crate::{bus::{Bus, Device}, cpu::{self, cs_type, Cpu, Insn}};

pub fn c_sw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_sw", rs1_p = insn.rs1_p(), rs2_p = insn.rs2_p(), c_uimm7lo = insn.c_uimm7lo(), c_uimm7hi = insn.c_uimm7hi());

    let rs1_p = insn.rs1_p() + 8;
    let rs2_p = insn.rs2_p() + 8;
    let c_uimm7lo = insn.c_uimm7lo();
    let c_uimm7hi = insn.c_uimm7hi();

    let imm = (c_uimm7lo & 0x1) << 6 | c_uimm7hi << 3 | (c_uimm7lo & 0x2) << 1;
    crate::trace_insn(cpu.pc, insn.bits(), "c.sw", cs_type!(rs1_p, rs2_p, imm));

    let address = cpu.load(rs1_p).wrapping_add(imm);
    bus.write(address, &(cpu.load(rs2_p) as u32).to_le_bytes())?;
    Ok(cpu.pc + 2)
}