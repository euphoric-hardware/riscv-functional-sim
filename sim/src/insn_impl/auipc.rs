use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn auipc(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("auipc", rd = insn.rd(), imm20 = insn.imm20());

    let rd = insn.rd();
    let imm20 = insn.imm20();

    cpu.regs[rd as usize] = Insn::sign_extend((cpu.pc + (imm20 << 12)) as u64, 32) as u64;
    Ok(cpu.pc + 4)
}
