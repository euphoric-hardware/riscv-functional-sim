use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn jal(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("jal", rd = insn.rd(), jimm20 = insn.jimm20());

    let rd = insn.rd();
    let jimm20 = insn.jimm20();

    let jimm20_sign_extended: i64 = Insn::sign_extend(
        ((jimm20 & 0x80000) << 1 | ((jimm20 & 0xff) << 12) | ((jimm20 & 0x7fe00) >> 9 << 1) | ((jimm20 & 0x100) >> 8 << 11)) as u64,
        20,
    );

    cpu.regs.store(rd, cpu.pc + 4);
    Ok(cpu.pc.wrapping_add(jimm20_sign_extended as u64) as u64)
}
