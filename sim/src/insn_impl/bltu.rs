use crate::{
    bus::Bus,
    cpu::{self, b_type, Cpu, Insn},
    log,
};

pub fn bltu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let bimm12hi = insn.bimm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let bimm12lo = insn.bimm12lo();

    let offset = Insn::sign_extend(
        ((((bimm12hi) & 0x7f) << 5)
            | ((bimm12lo) & 0x1 << 10)
            | (((bimm12hi) & 0x3f) << 5)
            | (bimm12lo) & 0x1e) as u64,
        12,
    );
    crate::trace_insn("bltu", b_type!(rs1, rs2, offset));

    if (cpu.regs.load(rs1) as u64) < (cpu.regs.load(rs2) as u64) {
        Ok((cpu.pc as i64 + offset) as u64)
    } else {
        Ok(cpu.pc + 4)
    }
}
