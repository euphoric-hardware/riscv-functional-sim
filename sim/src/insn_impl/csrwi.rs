use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn csrwi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!(
        "csrwi",
        rd = insn.rd(),
        csr = insn.csr(),
        zimm = insn.zimm()
    );

    let rd = insn.rd();
    let csr = insn.csr();
    let zimm = insn.zimm() as u64;

    cpu.csrs.store(csr, zimm)?;

    if rd != 0 {
        cpu.regs[rd as usize] = zimm;
    }

    Ok(cpu.pc + 4)
}
