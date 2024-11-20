use crate::{
    bus::Bus,
    cpu::{self, r_type, Cpu, Insn},
};

pub fn subw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    crate::trace_insn("subw", r_type!(rd, rs1, rs2));

    cpu.regs.store(
        rd,
        Insn::sign_extend(
            (cpu.regs.load(rs1) as u32).wrapping_sub(cpu.regs.load(rs2) as u32) as u64,
            32,
        ) as u64,
    );
    Ok(cpu.pc + 4)
}
