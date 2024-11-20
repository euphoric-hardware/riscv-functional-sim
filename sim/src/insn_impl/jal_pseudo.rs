use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn jal_pseudo(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("jal_pseudo", jimm20 = insn.jimm20());

    let jimm20 = insn.jimm20();

    Ok(cpu.pc + 4)
}
