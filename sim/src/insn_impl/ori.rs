use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn ori(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("ori", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(imm12 as u64, 12);
    cpu.regs.store(rd, cpu.regs.load(rs1) | (imm12_sign_extended as u64));
    Ok(cpu.pc + 4)
}
