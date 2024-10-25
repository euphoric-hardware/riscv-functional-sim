use crate::cpu::{Cpu, Insn};

pub fn lwu(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("lwu", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(imm12 as u64, 12);
    let address: usize = (cpu.regs[rs1 as usize] as u64).wrapping_add(imm12_sign_extended as u64) as usize;
    cpu.regs[rd as usize] = ((cpu.dram[address + 3] as u32) << 24
        | (cpu.dram[address + 2] as u32) << 16
        | (cpu.dram[address + 1] as u32) << 8
        | (cpu.dram[address] as u32)) as u64;
    cpu.pc += 4;
}