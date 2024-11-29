use crate::{
    bus::Bus,
    cpu::{self, r_type, Cpu, Insn},
};

pub fn slt(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    crate::trace_insn(cpu.pc, insn.bits(), "slt", r_type!(rd, rs1, rs2));

    cpu.store(rd, ((cpu.load(rs1) as i64) < (cpu.load(rs2) as i64)) as u64);
    Ok(cpu.pc + 4)
}
