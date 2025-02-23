use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn flw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12 as u64, 12);
    let address = (cpu.load(rs1) as u64).wrapping_add(imm as u64);

    let mut raw = [0; size_of::<f32>()];
    bus.read(address, &mut raw)?;
    let h = f32::from_le_bytes(raw) as f64;

    cpu.fstore(rd, h);
    Ok(cpu.pc + 4)
}