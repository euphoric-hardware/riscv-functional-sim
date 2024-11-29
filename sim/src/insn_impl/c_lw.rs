use crate::{bus::{Bus, Device}, cpu::{self, cl_type, Cpu, Insn}};

pub fn c_lw(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_lw", rd_p = insn.rd_p(), rs1_p = insn.rs1_p(), c_uimm7lo = insn.c_uimm7lo(), c_uimm7hi = insn.c_uimm7hi());

    let rd_p = insn.rd_p() + 8;
    let rs1_p = insn.rs1_p() + 8;
    let c_uimm7lo = insn.c_uimm7lo();
    let c_uimm7hi = insn.c_uimm7hi();
    
    let imm = (c_uimm7lo & 0x1) << 6 | c_uimm7hi << 3 | (c_uimm7lo & 0x2) << 1;
    crate::trace_insn(cpu.pc, insn.bits(), "c.lw", cl_type!(rd_p, rs1_p, imm));
    
    let address = (cpu.load(rs1_p) as u64).wrapping_add(imm);
    let mut raw = [0; size_of::<i32>()];
    bus.read(address, &mut raw)?;
    cpu.store(rd_p, i32::from_le_bytes(raw) as u64); // check sign extension

    Ok(cpu.pc + 2)
}