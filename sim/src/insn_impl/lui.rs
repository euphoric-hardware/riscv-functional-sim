use crate::{
    bus::Bus,
    cpu::{self, u_type, Cpu, Insn},
};

pub fn lui(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let imm20 = insn.imm20();

    let imm = Insn::sign_extend(imm20 << 12, 32) as u64;
    crate::trace_insn(cpu.pc, insn.bits(), "lui", u_type!(rd, imm));

    cpu.store(rd, imm);
    Ok(cpu.pc + 4)
}
