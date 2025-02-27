use crate::{
    bus::Bus,
    cpu::{self, u_type, Cpu, Insn},
};

pub fn auipc(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let imm20 = insn.imm20();

    let imm = Insn::sign_extend((imm20 << 12), 32) as u64;
    let value = cpu.pc.wrapping_add(imm);
    
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}
