use crate::{
    bus::Bus,
    cpu::{self, r_type, Cpu, Insn},
};

pub fn sraw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    crate::trace_insn("sraw", r_type!(rd, rs1, rs2));

    cpu.store(
        rd,
        Insn::sign_extend(
            (cpu.load(rs1) as i32).wrapping_shr((cpu.load(rs2) & 0x31) as u32) as u64,
            32,
        ) as u64,
    );
    Ok(cpu.pc + 4)
}
