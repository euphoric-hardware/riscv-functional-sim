use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn auipc(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("auipc", rd = insn.rd(), imm20 = insn.imm20());

    let rd = insn.rd();
    let imm20 = insn.imm20();
    cpu.regs
        .store(rd, cpu.pc.overflowing_add(imm20.overflowing_shl(12).0).0);

    Ok(cpu.pc + 4)
}
