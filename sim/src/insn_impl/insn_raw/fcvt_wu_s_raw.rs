use simple_soft_float::{FPState, StatusFlags, F32};

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn, RoundingMode},
    csrs::Csrs,
};

pub fn fcvt_wu_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let mut result: u32;
    let mode = Insn::get_rounding_mode(cpu, rm);
    let op1 = f32::from_bits(cpu.fload(rs1).to_bits() as u32);

    if (op1 > u32::MAX as f32) {
        result = u32::MAX;
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else if (op1 < i32::MIN as f32) {
        result = u32::MIN;
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else if (op1.is_nan()) {
        result = u32::MAX;
        cpu.csrs.store(Csrs::FFLAGS, 16);
    } else {
        cpu.update_hardware_fp_flags();
        #[cfg(target_arch = "aarch64")]
        {
            unsafe {
                core::arch::asm!("fmov d0, {0}", in(reg) op1);
                match mode {
                    Some(RoundingMode::RNE) => core::arch::asm!("fcvtnu {}, s0", out(reg) result),
                    Some(RoundingMode::RTZ) => core::arch::asm!("fcvtzu {}, s0", out(reg) result),
                    Some(RoundingMode::RDN) => core::arch::asm!("fcvtmu {}, s0", out(reg) result),
                    Some(RoundingMode::RUP) => core::arch::asm!("fcvtpu {}, s0", out(reg) result),
                    Some(RoundingMode::RMM) => core::arch::asm!("fcvtau {}, s0", out(reg) result),
                    None => todo!(),
                };
            }
        }
        cpu.set_fflags();
    }
    cpu.store(rd, result as i32 as i64 as u64);
    Ok(cpu.pc + 4)
}
