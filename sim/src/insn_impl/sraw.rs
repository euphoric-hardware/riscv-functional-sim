use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sraw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let result = Insn::sign_extend((cpu.load(rs1) as i64 as i32).wrapping_shr((cpu.load(rs2) & 0x1f) as u32) as u64, 32) as u64;
    cpu.store(rd, result);
    Ok(cpu.pc + 4)
}
