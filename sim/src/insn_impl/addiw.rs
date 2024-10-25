use crate::cpu::{Cpu, Insn};

pub fn addiw(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("addiw", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(imm12 as u64,12);
    let result = (rs1 as u32).wrapping_add(imm12_sign_extended as u32) as u64;
    cpu.regs[rd as usize] = Insn::sign_extend(result as u64, 32) as u64;
    cpu.pc += 4;
}