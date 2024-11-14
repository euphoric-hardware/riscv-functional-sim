use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn slti(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!(
        "slti",
        rd = insn.rd(),
        rs1 = insn.rs1(),
        imm12 = insn.imm12()
    );

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(imm12 as u64, 12);
    cpu.regs.store(
        rd,
        if (cpu.regs.load(rs1) as i64) < imm12_sign_extended {
            1
        } else {
            0
        },
    );
    Ok(cpu.pc + 4)
}
