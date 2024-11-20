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
        ((bimm12hi & 0x40) << 6)     // bit 7 of bimm12hi to imm[12]
        | ((bimm12lo & 0x01) << 11)  // bit 0 of bimm12lo to imm[11]
        | ((bimm12hi & 0x3F) << 5)   // bits 6:1 of bimm12hi to imm[10:5]
        | ((bimm12lo & 0x1E) >> 1)   // bits 4:1 of bimm12lo to imm[4:1]
        as u64,
        12,
    );

    crate::trace_insn("bne", b_type!(rs1, rs2, offset));

    if cpu.regs.load(rs1) != cpu.regs.load(rs2) {
        Ok((cpu.pc as i64 + offset) as u64)
    } else {
        Ok(cpu.pc + 4)
    }
}
