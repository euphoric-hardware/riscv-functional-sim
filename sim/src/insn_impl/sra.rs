pub fn sra(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("sra", rs1 = insn.rs1(), rs2 = insn.rs2(), rd = insn.rd());

    let rs1_val = cpu.regs[insn.rs1() as usize] as i64;
    let rs2_val = cpu.regs[insn.rs2() as usize] & 0x3f; // Shift amount is limited to 6 bits

    cpu.regs[insn.rd() as usize] = (rs1_val >> rs2_val) as u64;
}
