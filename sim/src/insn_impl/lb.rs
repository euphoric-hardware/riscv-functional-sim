use crate::cpu::{Cpu, Insn};

pub fn lb(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("lb", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(imm12 as u64, 12);
    let address: usize = (cpu.regs[rs1 as usize] as u64).wrapping_add(imm12_sign_extended as u64) as usize;
    cpu.regs[rd as usize] = (cpu.dram[address] as i64) as u64; // check sign extension, does casting the byte work?
    cpu.pc += 4;
}