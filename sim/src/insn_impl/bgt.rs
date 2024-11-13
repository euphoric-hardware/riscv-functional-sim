use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn bgt(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("bgt", bimm12hi = insn.bimm12hi(), rs2 = insn.rs2(), rs1 = insn.rs1(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs2 = insn.rs2();
    let rs1 = insn.rs1();
    let bimm12lo = insn.bimm12lo();

    Ok(cpu.pc + 4)
}