use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sll(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("sll", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    cpu.regs[rd as usize] =
        cpu.regs[rs1 as usize].wrapping_shl((cpu.regs[rs2 as usize] & 0x31) as u32);
    Ok(cpu.pc + 4)
}
