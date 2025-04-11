use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_addi4spn_raw(cpu: &mut Cpu, rd_p: u64, imm_c_addi4spn: u64) -> cpu::Result<u64> {
    let result = cpu.load(2).wrapping_add(imm_c_addi4spn);
    cpu.store(rd_p + 8, result);
    Ok(cpu.pc + 2)
}