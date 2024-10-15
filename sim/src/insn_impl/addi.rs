pub fn addi(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("addi", rs1 = insn.rs1(), rd = insn.rd(), imm = insn.s_imm());

    let rs1_val = cpu.regs[insn.rs1() as usize];
    let imm = insn.s_imm();

    cpu.regs[insn.rd() as usize] = rs1_val.wrapping_add(imm);
}
