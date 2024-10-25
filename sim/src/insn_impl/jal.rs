use crate::cpu::{Cpu, Insn};

pub fn jal(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("jal", rd = insn.rd(), jimm20 = insn.jimm20());

    let rd = insn.rd();
    let jimm20 = insn.jimm20();

    let jimm20_sign_extended: i64 = Insn::sign_extend(
        (jimm20 & 0x100000 | (jimm20 & 0xff << 12) | (jimm20 & 0xffe00 >> 8) | (jimm20 & 0x100 << 3)) as u64,
        20,
    );

    let stored_pc: u64 = cpu.pc + 4;
    let new_pc: u64 = cpu.pc + jimm20_sign_extended as u64;
    cpu.regs[rd as usize] = stored_pc;
    cpu.pc = new_pc;
}