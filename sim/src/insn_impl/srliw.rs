use crate::{
    bus::Bus,
    cpu::{self, i_type, Cpu, Insn},
};

pub fn srliw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtw = insn.shamtw();

    crate::trace_insn("srliw", i_type!(rd, rs1, shamtw));

    cpu.regs.store(
        rd,
        Insn::sign_extend((cpu.regs.load(rs1) as u32 >> shamtw) as u64, 32) as u64,
    );
    Ok(cpu.pc + 4)
}
