use crate::{
    bus::{Bus, Device},
    cpu::{self, i_type, Cpu, Insn},
};

pub fn lb(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm = Insn::sign_extend(imm12 as u64, 12);

    crate::trace_insn("lb", i_type!(rd, rs1, imm));

    let address = (cpu.regs.load(rs1) as u64).wrapping_add(imm as u64);
    let mut raw = [0];
    bus.read(address, &mut raw)?;
    cpu.regs.store(rd, (raw[0] as i64) as u64); // check sign extension, does casting the byte work?
    Ok(cpu.pc + 4)
}
