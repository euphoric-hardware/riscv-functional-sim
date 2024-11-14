use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn srl(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("srl", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    cpu.regs.store(
        rd,
        cpu.regs
            .load(rs1)
            .wrapping_shr((cpu.regs.load(rs2) & 0x31) as u32),
    );
    Ok(cpu.pc + 4)
}
