use crate::{bus::{Bus, Device}, cpu::{self, css_type, Cpu, Insn}};

pub fn c_swsp(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_swsp", c_rs2 = insn.c_rs2(), c_uimm8sp_s = insn.c_uimm8sp_s());

    let c_rs2 = insn.c_rs2();
    let c_uimm8sp_s = insn.c_uimm8sp_s();

    let offset = (c_uimm8sp_s & 0x3) << 6 | c_uimm8sp_s & 0x3c;

    crate::trace_insn(cpu.pc, insn.bits(), "c.swsp", css_type!(c_rs2, offset));

    let address = cpu.load(2).wrapping_add(offset as u64);
    bus.write(address, &(cpu.load(c_rs2) as u32).to_le_bytes())?;
    Ok(cpu.pc + 2)
}