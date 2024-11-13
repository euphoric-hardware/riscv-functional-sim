use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn csrw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrw", rd = insn.rd(), rs1 = insn.rs1(), csr = insn.csr());

    let rs1 = insn.rs1();
    let csr = insn.csr();

    let rs1_value = cpu.regs[rs1 as usize];
    cpu.csrs.store(csr, rs1_value)?;

    Ok(cpu.pc + 4)
}
