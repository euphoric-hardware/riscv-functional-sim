use crate::{bus::Bus, cpu::{self, Cpu, Insn}};

pub fn c_beqz(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_beqz", rs1_p = insn.rs1_p(), c_bimm9lo = insn.c_bimm9lo(), c_bimm9hi = insn.c_bimm9hi());

    let rs1_p = insn.rs1_p() + 8;
    let c_bimm9lo = insn.c_bimm9lo();
    let c_bimm9hi = insn.c_bimm9hi();

    let imm = (c_bimm9hi & 0x4) << 6 | (c_bimm9lo & 0x18) << 3 | (c_bimm9lo & 0x1) << 5 | (c_bimm9hi & 0x3) << 3 | c_bimm9lo & 0x6;

    
    let mut new_pc = cpu.pc + 2;

    if cpu.load(rs1_p) == 0 {
        new_pc = cpu.pc.wrapping_add(Insn::sign_extend(imm, 9) as u64);
    }

    Ok(new_pc)
}