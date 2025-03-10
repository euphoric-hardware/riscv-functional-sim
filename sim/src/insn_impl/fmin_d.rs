use simple_soft_float::F64;

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fmin_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let result = F64::from_bits(
        f64::min(
            f64::from_bits(*cpu.fload(rs1).bits()),
            f64::from_bits(*cpu.fload(rs2).bits()),
        )
        .to_bits(),
    );
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}
