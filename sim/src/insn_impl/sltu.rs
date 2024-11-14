use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sltu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("sltu", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    cpu.regs.store(rd, ((cpu.regs.load(rs1)) < (cpu.regs.load(rs2))) as u64);
    Ok(cpu.pc + 4)
}
