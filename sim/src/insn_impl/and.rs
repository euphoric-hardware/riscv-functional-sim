use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn and(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("and", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    cpu.regs[rd as usize] = cpu.regs[rs1 as usize] & cpu.regs[rs2 as usize];
    Ok(cpu.pc + 4)
}
