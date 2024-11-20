use crate::{
    bus::{Bus, Device},
    cpu::{self, i_type, Cpu, Insn},
};

pub fn lh(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12 as u64, 12);

    crate::trace_insn("lh", i_type!(rd, rs1, imm));

    let address = (cpu.regs.load(rs1) as u64).wrapping_add(imm as u64);

    let mut raw = [0; size_of::<i16>()];
    bus.read(address, &mut raw)?;
    let h = u16::from_le_bytes(raw);
    cpu.regs.store(rd, h as i16 as u64); // check sign extension
    Ok(cpu.pc + 4)
}
