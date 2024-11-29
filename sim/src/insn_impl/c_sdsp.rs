use crate::{bus::{Bus, Device}, cpu::{self, css_type, Cpu, Insn}};

pub fn c_sdsp(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_sdsp", c_rs2 = insn.c_rs2(), c_uimm9sp_s = insn.c_uimm9sp_s());

    let c_rs2 = insn.c_rs2();
    let c_uimm9sp_s = insn.c_uimm9sp_s();


    let imm = (c_uimm9sp_s & 0x7) << 6 | c_uimm9sp_s & 0x38;

    crate::trace_insn(cpu.pc, insn.bits(), "c.sdsp", css_type!(c_rs2, imm));

    let address = imm.wrapping_add(cpu.load(2));
    bus.write(address, &cpu.load(c_rs2).to_le_bytes());

    Ok(cpu.pc + 2)
}