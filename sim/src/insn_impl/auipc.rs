use crate::{
    bus::Bus,
    cpu::{self, u_type, Cpu, Insn},
};

pub fn auipc(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let imm20 = insn.imm20();

    let imm = Insn::sign_extend((imm20 as u64) << 12, 32);

    crate::trace_insn(cpu.pc, insn.bits(), "auipc", u_type!(rd, imm as u64));

    cpu.store(rd, cpu.pc.wrapping_add_signed(imm));

    Ok(cpu.pc + 4)
}
