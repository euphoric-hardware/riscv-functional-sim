use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn srli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtd = insn.shamtd();

    

    cpu.store(rd, cpu.load(rs1) >> shamtd);
    Ok(cpu.pc + 4)
}
