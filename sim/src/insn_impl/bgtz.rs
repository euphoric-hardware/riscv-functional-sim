use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn bgtz(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("bgtz", bimm12hi = insn.bimm12hi(), rs2 = insn.rs2(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs2 = insn.rs2();
    let bimm12lo = insn.bimm12lo();

    todo!();
}