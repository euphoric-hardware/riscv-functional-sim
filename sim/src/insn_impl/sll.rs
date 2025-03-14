use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn sll(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    

    cpu.store(
        rd,
        cpu.load(rs1).wrapping_shl((cpu.load(rs2) & 0x31) as u32),
    );
    Ok(cpu.pc + 4)
}
