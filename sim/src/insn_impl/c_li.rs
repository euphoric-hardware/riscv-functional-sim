use crate::{bus::Bus, cpu::{self, ci_type, Cpu, Insn}};

pub fn c_li(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_n0 = insn.rd_n0();
    let c_imm6lo = insn.c_imm6lo();
    let c_imm6hi = insn.c_imm6hi();

    let imm = Insn::sign_extend(c_imm6hi << 5 | c_imm6lo, 6);

    crate::trace_insn(cpu.pc, insn.bits(), "c.li", ci_type!(rd_n0, imm));
    cpu.store(rd_n0, imm as u64);
    Ok(cpu.pc + 2)
}