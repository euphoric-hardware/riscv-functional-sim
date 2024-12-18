use crate::{
    bus::Bus,
    cpu::{self, csr_reg_type, Cpu, Insn},
};

pub fn csrrw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let csr = insn.csr();

    crate::trace_insn("csrrw", csr_reg_type!(rd, csr, rs1));

    let csr_value = cpu.csrs.load(csr)?;
    let rs1_value = cpu.load(rs1);
    cpu.csrs.store(csr, rs1_value)?;
    cpu.store(rd, csr_value);
    Ok(cpu.pc + 4)
}
