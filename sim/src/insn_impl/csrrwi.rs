use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn csrrwi(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let csr = insn.csr();
    let zimm = insn.zimm() as u64;

    let csr_value = cpu.csrs.load(csr)?;
    cpu.csrs.store(csr, zimm)?;
    cpu.store(rd, csr_value);
    Ok(cpu.pc + 4)
}
