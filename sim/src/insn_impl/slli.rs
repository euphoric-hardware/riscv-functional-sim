pub fn slli(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("slli", rs1 = insn.rs1(), rd = insn.rd(), imm = insn.rs2());

    let rs1_val = cpu.regs[insn.rs1() as usize];
    let imm = insn.rs2() & 0x3f; // Shift amount limited to 6 bits

    cpu.regs[insn.rd() as usize] = rs1_val << imm;
}
