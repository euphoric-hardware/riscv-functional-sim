use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn srli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!(
        "srli",
        rd = insn.rd(),
        rs1 = insn.rs1(),
        shamtd = insn.shamtd()
    );

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtd = insn.shamtd();

    cpu.regs[rd as usize] = cpu.regs[rs1 as usize] >> shamtd;
    Ok(cpu.pc + 4)
}
