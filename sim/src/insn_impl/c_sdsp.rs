use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_sdsp(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_sdsp", c_rs2 = insn.c_rs2(), c_uimm9sp_s = insn.c_uimm9sp_s());

    let c_rs2 = insn.c_rs2();
    let c_uimm9sp_s = insn.c_uimm9sp_s();


    let imm = (c_uimm9sp_s & 0x7) << 6 | c_uimm9sp_s & 0x38;

    

    let address = cpu.load(2).wrapping_add(imm);
    bus.write(address, &cpu.load(c_rs2).to_le_bytes())?;

    Ok(cpu.pc + 2)
}