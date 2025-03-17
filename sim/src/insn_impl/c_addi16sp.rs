use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
};

pub fn c_addi16sp(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    // crate::trace_insn!("c_addi16sp", c_nzimm10hi = insn.c_nzimm10hi(), c_nzimm10lo = insn.c_nzimm10lo());

    let c_nzimm10hi = insn.c_nzimm10hi();
    let c_nzimm10lo = insn.c_nzimm10lo();

    let imm = Insn::sign_extend(
        c_nzimm10hi << 9
            | (c_nzimm10lo & 0x6) << 6
            | (c_nzimm10lo & 0x8) << 3
            | (c_nzimm10lo & 0x1) << 5
            | (c_nzimm10lo) & 0x10,
        10,
    );
    let result = cpu.load(2).wrapping_add(imm as u64);

    cpu.store(2, result);
    Ok(cpu.pc + 2)
}
