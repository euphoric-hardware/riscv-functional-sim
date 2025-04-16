use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_fsdsp(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let c_rs2 = insn.c_rs2();
    let c_uimm9sp_s = insn.c_uimm9sp_s();

    let imm = (c_uimm9sp_s & 0x7) << 6 | (c_uimm9sp_s & 0x38);
    let address = (cpu.load(2) as u64).wrapping_add(imm as u64);
    let result = cpu.fload(c_rs2);
    
    bus.write(address, &result.to_le_bytes());
    Ok(cpu.pc + 2)
}