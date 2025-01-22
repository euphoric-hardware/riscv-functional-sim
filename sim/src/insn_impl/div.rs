use crate::{bus::Bus, cpu::{self, r_type, Cpu, Insn}};

pub fn div(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    crate::trace_insn(cpu.pc, insn.bits(), "div", r_type!(rd, rs1, rs2));

    let rs1_value = cpu.load(rs1) as i64;
    let rs2_value = cpu.load(rs2) as i64;

    if rs2_value == 0 {
        cpu.store(rd, u64::MAX);
    } else if rs1_value == i64::MIN && rs2_value == -1 {
        cpu.store(rd, rs1_value as u64);
    } else {
        cpu.store(rd, (rs1_value / rs2_value) as u64);
    }

    Ok(cpu.pc + 4)
}