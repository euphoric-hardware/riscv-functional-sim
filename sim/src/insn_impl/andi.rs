use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    log,
};

pub fn andi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12, 12);

    cpu.store(rd, cpu.load(rs1) & imm as u64);
    Ok(cpu.pc + 4)
}
