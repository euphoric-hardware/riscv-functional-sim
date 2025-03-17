use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_addi4spn(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_addi4spn", rd_p = insn.rd_p(), c_nzuimm10 = insn.c_nzuimm10());

    let rd_p = insn.rd_p() + 8;
    let c_nzuimm10 = insn.c_nzuimm10();

    let imm = (c_nzuimm10 & 0xc0) >> 2
        | (c_nzuimm10 & 0x3c) << 4
        | (c_nzuimm10 & 0x02) << 1
        | (c_nzuimm10 & 0x01) << 3;
    let result = cpu.load(2).wrapping_add(imm);

    cpu.store(rd_p, result);

    Ok(cpu.pc + 2)
}
