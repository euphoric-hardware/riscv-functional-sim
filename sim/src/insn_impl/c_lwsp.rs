use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
};

pub fn c_lwsp(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_lwsp", rd_n0 = insn.rd_n0(), c_uimm8sphi = insn.c_uimm8sphi(), c_uimm8splo = insn.c_uimm8splo());

    let rd_n0 = insn.rd_n0();
    let c_uimm8sphi = insn.c_uimm8sphi();
    let c_uimm8splo = insn.c_uimm8splo();

    let offset = c_uimm8sphi << 5 | (c_uimm8splo & 0x3) << 6 | c_uimm8splo & 0x1c;

    let address = (cpu.load(2)).wrapping_add(offset as u64);
    let mut raw = [0; size_of::<i32>()];

    bus.read(address, &mut raw)?;
    cpu.store(rd_n0, i32::from_le_bytes(raw) as u64); // check sign extension
    Ok(cpu.pc + 2)
}
