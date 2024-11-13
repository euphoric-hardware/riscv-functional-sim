use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sraiw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!(
        "sraiw",
        rd = insn.rd(),
        rs1 = insn.rs1(),
        shamtw = insn.shamtw()
    );

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtw = insn.shamtw();

    cpu.regs[rd as usize] = ((cpu.regs[rs1 as usize] as i32) >> shamtw) as i64 as u64;
    Ok(cpu.pc + 4)
}
