use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn divw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let rs1_value = cpu.load(rs1) as u32 as i32;
    let rs2_value = cpu.load(rs2) as u32 as i32;

    if rs2_value == 0 {
        cpu.store(rd, u64::MAX);
    } else {
        cpu.store(rd, (rs1_value / rs2_value) as i64 as u64);
    }

    Ok(cpu.pc + 4)
}
