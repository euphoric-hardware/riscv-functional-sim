use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sra(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    

    cpu.store(
        rd,
        (cpu.load(rs1) as i64).wrapping_shr((cpu.load(rs2) & 0x31) as u32) as u64,
    );
    Ok(cpu.pc + 4)
}
