use simple_soft_float::{FPState, StatusFlags, F32};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn, RoundingMode},
    csrs::Csrs,
};

pub fn fcvt_l_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let mut result: i64 = 0;
    let mode = Insn::get_rounding_mode(cpu, rm);
    let op1 = f32::from_bits(cpu.fload(rs1).to_bits() as u32);

    if (op1 > i64::MAX as f32) {
        result = i64::MAX;
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else if (op1 < i64::MIN as f32) {
        result = i64::MIN;
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else if (op1.is_nan()) {
        result = i64::MAX;
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else {
        cpu.update_hardware_fp_flags();
        #[cfg(target_arch = "aarch64")]
    {
        unsafe {
            core::arch::asm!("fmov d0, {0}", in(reg) op1);
            match mode {
                Some(RoundingMode::RNE) => core::arch::asm!("fcvtns {}, s0", out(reg) result),
                Some(RoundingMode::RTZ) => core::arch::asm!("fcvtzs {}, s0", out(reg) result),
                Some(RoundingMode::RDN) => core::arch::asm!("fcvtms {}, s0", out(reg) result),
                Some(RoundingMode::RUP) => core::arch::asm!("fcvtps {}, s0", out(reg) result),
                Some(RoundingMode::RMM) => core::arch::asm!("fcvtas {}, s0", out(reg) result),
                None => todo!(),
            };
        }
    }
        cpu.set_fflags();
    }
    cpu.store(rd, result as u64);
    Ok(cpu.pc + 4)
}
