use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn}
};

#[inline(always)]
pub fn lb_raw(cpu: &mut Cpu, bus: &mut Bus, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(imm_i as u64, 12);

    let address = (cpu.load(rs1) as u64).wrapping_add(offset as u64);
    let mut raw = [0];
    bus.read(address, &mut raw)?;
    cpu.store(rd, (raw[0] as i8) as u64); // check sign extension, does casting the byte work?
    Ok(cpu.pc + 4)
}