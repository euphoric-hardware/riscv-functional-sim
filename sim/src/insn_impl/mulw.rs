use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn mulw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let result = Insn::sign_extend(
        (cpu.load(rs1) as u32).wrapping_mul(cpu.load(rs2) as u32) as u64,
        32,
    );
    cpu.store(rd, result as u64);

    Ok(cpu.pc + 4)
}
