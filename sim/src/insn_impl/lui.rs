use crate::cpu::{Cpu, Insn};

pub fn lui(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("lui", rd = insn.rd(), imm20 = insn.imm20());

    let rd = insn.rd();
    let imm20 = insn.imm20();

    cpu.regs[rd as usize] = Insn::sign_extend((imm20 << 12) as u64, 32) as u64;
    cpu.pc += 4;
}