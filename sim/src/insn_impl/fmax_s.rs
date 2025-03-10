use simple_soft_float::{F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fmax_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let result = F64::from_bits(
        (f32::max(
            f32::from_bits(*F32::convert_from_float(&cpu.fload(rs1), None, None).bits()),
            f32::from_bits(*F32::convert_from_float(&cpu.fload(rs2), None, None).bits()),
        ) as f64)
            .to_bits(),
    );
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}
