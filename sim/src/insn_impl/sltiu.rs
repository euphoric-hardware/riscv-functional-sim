use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sltiu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12 as u64, 12);

    cpu.store(rd, if cpu.load(rs1) < (imm as u64) { 1 } else { 0 });
    Ok(cpu.pc + 4)
}
