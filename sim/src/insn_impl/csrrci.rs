use crate::{
    bus::Bus,
    cpu::{self, csr_imm_type, Cpu, Insn},
};

pub fn csrrci(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let csr = insn.csr();
    let zimm = insn.zimm() as u64;

    crate::trace_insn(cpu.pc, insn.bits(), "csrrci", csr_imm_type!(rd, csr, zimm));

    let csr_value = cpu.csrs.load(csr)?;
    cpu.csrs.store(csr, csr_value & !zimm)?;
    cpu.store(rd, csr_value);
    Ok(cpu.pc + 4)
}
