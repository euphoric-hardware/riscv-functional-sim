use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fsgnjx_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let sign_bit =
        ((f64::to_bits(cpu.fload(rs1)) & 0x8000000000000000) ^ (f64::to_bits(cpu.fload(rs2)) & 0x8000000000000000));
    let value = f64::from_bits(f64::to_bits(cpu.fload(rs1)) & 0x7fffffffffffffff | sign_bit);
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}
