use simple_soft_float::{F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fcvt_d_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let result =
        F64::convert_from_float(&F32::from_bits(*cpu.fload(rs1).bits() as u32), None, None);
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}
