use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

pub fn lb(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("lb", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(imm12 as u64, 12);
    let address = (cpu.regs.load(rs1) as u64).wrapping_add(imm12_sign_extended as u64);

    let mut raw = [0];
    bus.read(address, &mut raw)?;

    cpu.regs.store(rd, (raw[0] as i64) as u64); // check sign extension, does casting the byte work?
    Ok(cpu.pc + 4)
}
