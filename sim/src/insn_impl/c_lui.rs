use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_lui(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_lui", rd_n2 = insn.rd_n2(), c_nzimm18hi = insn.c_nzimm18hi(), c_nzimm18lo = insn.c_nzimm18lo());

    let rd_n2 = insn.rd_n2();
    let c_nzimm18hi = insn.c_nzimm18hi();
    let c_nzimm18lo = insn.c_nzimm18lo();

    let imm = Insn::sign_extend(c_nzimm18hi << 17 | c_nzimm18lo << 12, 18);

    cpu.store(rd_n2, imm as u64);
    Ok(cpu.pc + 2)
}
