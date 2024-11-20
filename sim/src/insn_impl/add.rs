use crate::{
    bus::Bus,
    cpu::{self, r_type, Cpu, Insn},
    log,
};

pub fn add(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    log::trace_insn("add", r_type!(rd, rs1, rs2));

    cpu.regs.store(rd, cpu.regs.load(rs1).wrapping_add(cpu.regs.load(rs2)));
    Ok(cpu.pc + 4)
}
