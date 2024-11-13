use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn csrr(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("csrr", rd = insn.rd(), csr = insn.csr());

    let rd = insn.rd();
    let csr = insn.csr();

    let csr_value = cpu.csrs.load(csr)?;

    if rd != 0 {
        cpu.regs[rd as usize] = csr_value;
    }

    Ok(cpu.pc + 4)
}
