use crate::{
    bus::{Bus, Device},
    cpu::{self, s_type, Cpu, Insn},
};

pub fn sb(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let imm12hi = insn.imm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let imm12lo = insn.imm12lo();

    let offset = Insn::sign_extend((imm12hi << 5 | imm12lo) as u64, 12);

    crate::trace_insn(cpu.pc, insn.bits(), "sb", s_type!(rs1, rs2, offset));

    let address = cpu.load(rs1).wrapping_add(offset as u64);
    let result = &(cpu.load(rs2) as u8).to_le_bytes();
    bus.write(address, result)?;
    Ok(cpu.pc + 4)
}
