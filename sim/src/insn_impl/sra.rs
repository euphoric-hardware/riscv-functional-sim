use crate::cpu::{Cpu, Insn};

pub fn sra(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("sra", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    cpu.regs[rd as usize] = (cpu.regs[rs1 as usize] as i64).wrapping_shr((cpu.regs[rs2 as usize] & 0x31) as u32) as u64;
    cpu.pc += 4;
}