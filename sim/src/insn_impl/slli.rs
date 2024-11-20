use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn slli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("slli", rd = insn.rd(), rs1 = insn.rs1(), shamtd = insn.shamtd());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtd = insn.shamtd();

    cpu.regs.store(rd, cpu.regs.load(rs1) << shamtd);
    Ok(cpu.pc + 4)
}
