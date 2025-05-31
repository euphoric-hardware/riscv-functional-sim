use simple_soft_float::F64;

use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn}
};

pub fn c_fldsp(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let c_uimm9sphi = insn.c_uimm9sphi();
    let c_uimm9splo = insn.c_uimm9splo();

    let imm = (c_uimm9sphi << 5) | c_uimm9splo & 0x18 | ((c_uimm9splo & 0x7) << 6);
    let address = cpu.load(2).wrapping_add(imm);

    let mut raw = [0; size_of::<f64>()];
    bus.read(address, &mut raw)?;
    let h = f64::from_le_bytes(raw);
    cpu.fstore(rd, h);
    Ok(cpu.pc + 2)
}
