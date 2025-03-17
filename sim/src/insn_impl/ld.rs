use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

pub fn ld(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12 as u64, 12);

    let address = (cpu.load(rs1) as u64).wrapping_add(imm as u64);
    let mut raw = [0; size_of::<u64>()];
    bus.read(address, &mut raw)?;
    cpu.store(rd, u64::from_le_bytes(raw)); // check sign extension, does casting the byte work?
    Ok(cpu.pc + 4)
}
