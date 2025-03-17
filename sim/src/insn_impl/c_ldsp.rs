use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

pub fn c_ldsp(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_ldsp", rd_n0 = insn.rd_n0(), c_uimm9sphi = insn.c_uimm9sphi(), c_uimm9splo = insn.c_uimm9splo());

    let rd_n0 = insn.rd_n0();
    let c_uimm9sphi = insn.c_uimm9sphi();
    let c_uimm9splo = insn.c_uimm9splo();

    let imm = (c_uimm9splo & 0x7) << 6 | c_uimm9sphi << 5 | (c_uimm9splo & 0x18);

    let mut raw = [0; size_of::<u64>()];
    bus.read(cpu.load(2).wrapping_add(imm), &mut raw)?;
    cpu.store(rd_n0, u64::from_le_bytes(raw));

    Ok(cpu.pc + 2)
}
