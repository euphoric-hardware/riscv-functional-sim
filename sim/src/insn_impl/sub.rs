use crate::{
    bus::Bus,
    cpu::{self, r_type, Cpu, Insn},
};

pub fn sub(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    crate::trace_insn(cpu.pc, insn.bits(), "sub", r_type!(rd, rs1, rs2));

    cpu.store(rd, cpu.load(rs1).wrapping_sub(cpu.load(rs2)));
    Ok(cpu.pc + 4)
}
