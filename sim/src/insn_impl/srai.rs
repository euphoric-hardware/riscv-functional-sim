use crate::{
    bus::Bus,
    cpu::{self, i_type, Cpu, Insn},
};

pub fn srai(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtd = insn.shamtd();

    crate::trace_insn(cpu.pc, insn.bits(), "srai", i_type!(rd, rs1, shamtd));

    cpu.store(rd, ((cpu.load(rs1) as i64) >> shamtd) as u64);
    Ok(cpu.pc + 4)
}
