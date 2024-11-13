use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn lui(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("lui", rd = insn.rd(), imm20 = insn.imm20());

    let rd = insn.rd();
    let imm20 = insn.imm20();

    cpu.regs[rd as usize] = Insn::sign_extend((imm20 << 12) as u64, 32) as u64;
    Ok(cpu.pc + 4)
}
