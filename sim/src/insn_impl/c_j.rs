use crate::{
    bus::Bus,
    cpu::{self, cj_type, Cpu, Insn},
};

pub fn c_j(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_j", c_imm12 = insn.c_imm12());

    let c_imm12 = insn.c_imm12();
    let offset_raw: u64 = (c_imm12 & 0x400) << 1
        | (c_imm12 & 0x40) << 4
        | (c_imm12 & 0x180) << 1
        | (c_imm12 & 0x10) << 3
        | (c_imm12 & 0x20) << 1
        | (c_imm12 & 0x200) >> 5
        | (c_imm12 & 0xe)
        | (c_imm12 & 0x1) << 5;
    let offset = Insn::sign_extend(offset_raw, 12);

    crate::trace_insn(cpu.pc, insn.bits(), "c.j", cj_type!(offset));
    let new_pc = cpu.pc.wrapping_add(offset as u64);

    Ok(new_pc as u64)
}
