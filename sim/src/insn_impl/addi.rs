use crate::{
    bus::Bus,
    cpu::{self, i_type, Cpu, Insn},
    log,
};

pub fn addi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12 as u64, 12);

    log::trace_insn("addi", i_type!(rd, rs1, imm));

    cpu.regs
        .store(rd, cpu.regs.load(rs1).wrapping_add(imm as u64));
    Ok(cpu.pc + 4)
}
