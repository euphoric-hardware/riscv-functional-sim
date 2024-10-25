use crate::cpu::{Cpu, Insn};

pub fn addw(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("addw", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    cpu.regs[rd as usize] = Insn::sign_extend(
        (cpu.regs[rs1 as usize] as u32).wrapping_add(cpu.regs[rs2 as usize] as u32) as u64,
        32,
    ) as u64;
    cpu.pc += 4;
    
}
