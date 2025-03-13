use simple_soft_float::F32;

use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

pub fn fsw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let imm12hi = insn.imm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let imm12lo = insn.imm12lo();

    let imm = Insn::sign_extend((imm12hi << 5 | imm12lo) as u64, 12);
    let address = (cpu.load(rs1) as u64).wrapping_add(imm as u64);

    let result = *cpu.fload(rs2).bits() as u32;
    
    bus.write(address, &result.to_le_bytes());
    Ok(cpu.pc + 4)
}
