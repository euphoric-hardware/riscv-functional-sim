use simple_soft_float::{F32, F64};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fsgnjx_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let sign_bit = ((*F32::convert_from_float(&cpu.fload(rs1), None, None).bits()) & 0x80000000)
        ^ (*F32::convert_from_float(&cpu.fload(rs2), None, None).bits() & 0x80000000);
    let result = F32::from_bits(
        (*F32::convert_from_float(&cpu.fload(rs1), None, None).bits() & 0x7fffffff) | sign_bit,
    );
    let result64 = F64::convert_from_float(&result, None, None);
    cpu.fstore(rd, result64);
    Ok(cpu.pc + 4)
}
