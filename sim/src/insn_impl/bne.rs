use crate::{
    bus::Bus,
    cpu::{self, b_type, Cpu, Insn},
};

pub fn bne(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let bimm12hi = insn.bimm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let bimm12lo = insn.bimm12lo();

    let offset = Insn::sign_extend(
        ((bimm12hi & 0x40) << 5)
            | ((bimm12lo & 0x01) << 11)
            | ((bimm12hi & 0x3F) << 5)
            | (bimm12lo & 0x1E),
        12,
    );

    crate::trace_insn(cpu.pc, insn.bits(), "bne", b_type!(rs1, rs2, offset));
    if cpu.load(rs1) != cpu.load(rs2) {
        Ok((cpu.pc as i64 + offset) as u64)
    } else {
        Ok(cpu.pc + 4)
    }
}
