use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

pub fn sd(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!(
        "sd",
        imm12hi = insn.imm12hi(),
        rs1 = insn.rs1(),
        rs2 = insn.rs2(),
        imm12lo = insn.imm12lo()
    );

    let imm12hi = insn.imm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let imm12lo = insn.imm12lo();

    let imm12_sign_extended = Insn::sign_extend((imm12hi << 5 | imm12lo) as u64, 12);
    let address = cpu.regs.load(rs1) + imm12_sign_extended as u64;
    bus.write(address, &cpu.regs.load(rs2).to_le_bytes());

    Ok(cpu.pc + 4)
}
