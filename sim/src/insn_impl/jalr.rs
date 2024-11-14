use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn jalr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!(
        "jalr",
        rd = insn.rd(),
        rs1 = insn.rs1(),
        imm12 = insn.imm12()
    );

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(imm12, 12);
    let new_pc = cpu.regs.load(rs1) + imm12_sign_extended as u64;
    cpu.regs.store(rd, cpu.pc + 4);

    Ok(new_pc)
}
