use crate::{
    bus::Bus,
    cpu::{self, u_type, Cpu, Insn},
};

pub fn auipc(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let imm20 = insn.imm20();

    let imm = (imm20 as u64) << 12;

    crate::trace_insn("auipc", u_type!(rd, imm));

    cpu.regs.store(rd, cpu.pc.wrapping_add(imm));

    Ok(cpu.pc + 4)
}
