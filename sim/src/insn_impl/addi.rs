use crate::cpu::{Cpu, Insn};

pub fn addi(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("addi", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();
    
    let imm12_sign_extended = Insn::sign_extend(imm12 as u64,12);
    cpu.regs[rd as usize] = cpu.regs[rs1 as usize].wrapping_add(imm12_sign_extended as u64);
    cpu.pc += 4;
}