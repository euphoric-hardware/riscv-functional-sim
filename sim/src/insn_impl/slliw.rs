use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn slliw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!(
        "slliw",
        rd = insn.rd(),
        rs1 = insn.rs1(),
        shamtw = insn.shamtw()
    );

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtw = insn.shamtw();

    cpu.regs[rd as usize] =
        Insn::sign_extend(((cpu.regs[rs1 as usize] as u32) << shamtw) as u64, 32) as u64;
    Ok(cpu.pc + 4)
}
