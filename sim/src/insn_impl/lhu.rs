use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

pub fn lhu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12 as u64, 12);

    let address = (cpu.load(rs1) as u64).wrapping_add(imm as u64);
    let mut raw = [0; size_of::<u16>()];
    bus.read(address, &mut raw)?;
    let h = u16::from_le_bytes(raw);
    cpu.store(rd, h as u64);
    Ok(cpu.pc + 4)
}
