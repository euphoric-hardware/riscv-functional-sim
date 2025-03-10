use simple_soft_float::{F32, F64};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fcvt_s_lu(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let result =
        F64::convert_from_float(&F32::from_u64(cpu.load(rs1), None, None), None, None);
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}