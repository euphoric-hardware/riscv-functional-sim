use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sraiw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtw = insn.shamtw();

    

    cpu.store(rd, ((cpu.load(rs1) as i32) >> shamtw) as i64 as u64);
    Ok(cpu.pc + 4)
}
