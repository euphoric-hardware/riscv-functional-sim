use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn bgez(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("bgez", bimm12hi = insn.bimm12hi(), rs1 = insn.rs1(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs1 = insn.rs1();
    let bimm12lo = insn.bimm12lo();

    todo!();
}