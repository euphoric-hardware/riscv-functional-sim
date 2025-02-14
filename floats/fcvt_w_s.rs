use crate::{bus::Bus, cpu::{self, r_type, Cpu, Insn}};

pub fn fcvt_w_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rm = insn.rm();

    let value = cpu.fload(rs1) as i32 as f64;
    cpu.fstore(rd, value);

    Ok(cpu.pc + 4)
}