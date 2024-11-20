use crate::{
    bus::Bus,
    cpu::{self, i_type, Cpu, Insn},
};

pub fn srli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtd = insn.shamtd();

    crate::trace_insn("srli", i_type!(rd, rs1, shamtd));

    cpu.regs.store(rd, cpu.regs.load(rs1) >> shamtd);
    Ok(cpu.pc + 4)
}
