use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn fnmadd_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let rs3 = insn.rs3();
    let rm = insn.rm();

    let value = (-(cpu.fload(rs1) as f32 * cpu.fload(rs2) as f32) + cpu.fload(rs3) as f32) as f64;
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}
