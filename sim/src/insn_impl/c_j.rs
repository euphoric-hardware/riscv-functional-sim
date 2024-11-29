use crate::{bus::Bus, cpu::{self, cj_type, Cpu, Insn}};

pub fn c_j(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_j", c_imm12 = insn.c_imm12());

    let c_imm12 = insn.c_imm12();
    crate::trace_insn(cpu.pc, insn.bits(), "c.j", cj_type!(c_imm12));

    let offset = Insn::sign_extend((c_imm12 & 0x800) | (c_imm12 & 0x40) << 4 | (c_imm12 & 0x180) << 1 | (c_imm12 & 0x10) << 2 | (c_imm12 & 0x20) << 1 | (c_imm12 & 0x2) << 5 | (c_imm12 & 0x200) >> 5 | (c_imm12 & 0xe), 12);
    Ok(cpu.pc.wrapping_add_signed(offset))
}