use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn csrrsi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!(
        "csrrsi",
        rd = insn.rd(),
        csr = insn.csr(),
        zimm = insn.zimm()
    );

    let rd = insn.rd();
    let csr = insn.csr();
    let zimm = insn.zimm() as u64;

    let csr_value = cpu.csrs.load(csr)?;

    if zimm != 0 {
        cpu.csrs.store(csr, csr_value | zimm)?;
    }

    if rd != 0 {
        cpu.regs[rd as usize] = csr_value;
    }

    Ok(cpu.pc + 4)
}
