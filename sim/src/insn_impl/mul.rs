use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn mul(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let result = cpu.load(rs1).wrapping_mul(cpu.load(rs2));
    cpu.store(rd, result);

    Ok(cpu.pc + 4)
}
