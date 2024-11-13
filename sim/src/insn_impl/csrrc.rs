use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn csrrc(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrrc", rd = insn.rd(), rs1 = insn.rs1(), csr = insn.csr());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let csr = insn.csr();

    let csr_value = cpu.csrs.load(csr)?;

    if rs1 != 0 {
        let rs1_value = cpu.regs[rs1 as usize];
        cpu.csrs.store(csr, csr_value & !rs1_value)?;
    }

    if rd != 0 {
        cpu.regs[rd as usize] = csr_value;
    }

    Ok(cpu.pc + 4)
}
