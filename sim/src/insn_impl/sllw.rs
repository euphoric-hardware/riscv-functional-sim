use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sllw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("sllw", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    cpu.regs[rd as usize] = Insn::sign_extend(
        (cpu.regs[rs1 as usize] as u32).wrapping_shl((cpu.regs[rs2 as usize] & 0x31) as u32) as u64,
        32,
    ) as u64;
    Ok(cpu.pc + 4)
}
