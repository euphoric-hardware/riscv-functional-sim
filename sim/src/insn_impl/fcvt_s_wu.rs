use simple_soft_float::{F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fcvt_s_wu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let result = F64::from_bits((cpu.load(rs1) as u32 as f32).to_bits() as u64);
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}
