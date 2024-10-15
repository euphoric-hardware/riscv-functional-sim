pub fn sub(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("sub", rs1 = insn.rs1(), rs2 = insn.rs2(), rd = insn.rd());

    let rs1_val = cpu.regs[insn.rs1() as usize];
    let rs2_val = cpu.regs[insn.rs2() as usize];

    cpu.regs[insn.rd() as usize] = rs1_val.wrapping_sub(rs2_val);
}
