use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_srli(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd_rs1_p = insn.rd_rs1_p() + 8;
    let c_nzuimm5 = insn.c_nzuimm5();
    
    let shamt = (1 << 5) | c_nzuimm5;
    let result = cpu.load(rd_rs1_p).wrapping_shr(shamt as u32);
    cpu.store(rd_rs1_p, result);
    Ok(cpu.pc + 2)
}
