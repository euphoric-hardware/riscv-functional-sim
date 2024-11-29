use crate::{
    bus::Bus,
    cpu::{self, i_type, Cpu, Insn},
};

pub fn sltiu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12 as u64, 12);

    crate::trace_insn(cpu.pc, insn.bits(), "sltiu", i_type!(rd, rs1, imm));

    cpu.store(rd, if rs1 < (imm as u64) { 1 } else { 0 });
    Ok(cpu.pc + 4)
}
