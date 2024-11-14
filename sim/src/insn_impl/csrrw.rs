use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn csrrw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrrw", rd = insn.rd(), rs1 = insn.rs1(), csr = insn.csr());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let csr = insn.csr();

    let csr_value = cpu.csrs.load(csr)?;

    let rs1_value = cpu.regs.load(rs1);
    cpu.csrs.store(csr, rs1_value)?;

    if rd != 0 {
        cpu.regs.store(rd, csr_value);
    }

    Ok(cpu.pc + 4)
}
