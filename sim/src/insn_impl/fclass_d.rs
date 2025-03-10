use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fclass_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    let value = cpu.fload(rs1);
    let classification = value.class();
    let class = match classification {
        simple_soft_float::FloatClass::NegativeInfinity => 1 << 0,
        simple_soft_float::FloatClass::NegativeNormal => 1 << 1,
        simple_soft_float::FloatClass::NegativeSubnormal => 1 << 2,
        simple_soft_float::FloatClass::NegativeZero => 1 << 3,
        simple_soft_float::FloatClass::PositiveInfinity => 1 << 7,
        simple_soft_float::FloatClass::PositiveNormal => 1 << 6,
        simple_soft_float::FloatClass::PositiveSubnormal => 1 << 5,
        simple_soft_float::FloatClass::PositiveZero => 1 << 4,
        simple_soft_float::FloatClass::QuietNaN => 1 << 9,
        simple_soft_float::FloatClass::SignalingNaN => 1 << 8,
    };
    cpu.store(rd, class);
    Ok(cpu.pc + 4)
}