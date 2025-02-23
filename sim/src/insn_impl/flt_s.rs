use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn flt_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let value = if (cpu.fload(rs1) as f32) < (cpu.fload(rs2) as f32) {
        1.0
    } else {
        0.0
    };
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}
