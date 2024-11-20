use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn bltu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("bltu", bimm12hi = insn.bimm12hi(), rs1 = insn.rs1(), rs2 = insn.rs2(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let bimm12lo = insn.bimm12lo();

    if (cpu.regs.load(rs1)) < (cpu.regs.load(rs2)) {
        let bimm12_sign_extended = Insn::sign_extend(((((bimm12hi) & 0x7f) << 5) | ((bimm12lo) & 0x1 << 10) | (((bimm12hi) & 0x3f) << 5) | (bimm12lo) & 0x1e) as u64, 12);

        Ok((cpu.pc as i64 + (bimm12_sign_extended)) as u64)
    } else {
        Ok(cpu.pc + 4)
    }
}
