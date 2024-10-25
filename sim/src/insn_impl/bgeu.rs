use crate::cpu::{Cpu, Insn};

pub fn bgeu(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("bgeu", bimm12hi = insn.bimm12hi(), rs1 = insn.rs1(), rs2 = insn.rs2(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let bimm12lo = insn.bimm12lo();

    if (cpu.regs[rs1 as usize] as i64) == (cpu.regs[rs2 as usize] as i64) {
        let bimm12_sign_extended = Insn::sign_extend(((((bimm12hi) & 0x7f) << 5)
        | ((bimm12lo) & 0x1 << 10)
        | (((bimm12hi) & 0x3f) << 5)
        | (bimm12lo) & 0x1e) as u64, 12);

        cpu.pc = (cpu.pc as i64 + (bimm12_sign_extended)) as u64;
    }

    else {
        cpu.pc += 4;
    }
}