use crate::{
    bus::Bus,
    cpu::{self, i_type, Cpu, Insn},
};

pub fn jalr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12, 12);

    crate::trace_insn(cpu.pc, insn.bits(), "jalr", i_type!(rd, rs1, imm));

    let new_pc = cpu.load(rs1) + imm as u64;
    cpu.store(rd, cpu.pc + 4);
    Ok(new_pc)
}
