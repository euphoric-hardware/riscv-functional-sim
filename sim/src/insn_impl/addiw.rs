use crate::{
    bus::Bus,
    cpu::{self, i_type, Cpu, Insn},
};

pub fn addiw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12, 12);

    crate::trace_insn("addiw", i_type!(rd, rs1, imm));

    let result = (cpu.load(rs1) as u32).wrapping_add(imm as u32) as u64;
    cpu.store(rd, Insn::sign_extend(result as u64, 32) as u64);
    Ok(cpu.pc + 4)
}
