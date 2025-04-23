use simple_soft_float::{FPState, StatusFlags};

use crate::{bus::Bus, cpu::{self, Cpu, Insn, RoundingMode}, csrs::Csrs};

pub fn fcvt_lu_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let mut result: u64 = 0;
    let mode = Insn::get_rounding_mode(cpu, rm);
    cpu.update_hardware_fp_flags();
    #[cfg(target_arch = "aarch64")]
    {
    unsafe {
        core::arch::asm!("fmov d0, {0}", in(reg) cpu.fload(rs1));
        match mode {
            Some(RoundingMode::RNE) => core::arch::asm!("fcvtns {}, d0", out(reg) result),
            Some(RoundingMode::RTZ) => core::arch::asm!("fcvtzs {}, d0", out(reg) result),
            Some(RoundingMode::RDN) => core::arch::asm!("fcvtms {}, d0", out(reg) result),
            Some(RoundingMode::RUP) => core::arch::asm!("fcvtps {}, d0", out(reg) result),
            Some(RoundingMode::RMM) => core::arch::asm!("fcvtas {}, d0", out(reg) result),
            None => todo!(),
        };

    }
}
    cpu.set_fflags();
    cpu.store(rd, result as u64);
    Ok(cpu.pc + 4)
}