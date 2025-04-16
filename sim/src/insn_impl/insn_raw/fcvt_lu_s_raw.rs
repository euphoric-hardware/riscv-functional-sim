use simple_soft_float::{FPState, StatusFlags, F32};

use crate::{bus::Bus, cpu::{self, Cpu, Insn, RoundingMode}, csrs::Csrs};

pub fn fcvt_lu_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let result: u64;
    let mode = Insn::get_rounding_mode(cpu, rm);
    unsafe {
        let mut old_fpcr: u64;
        let mut new_fpcr: u64;

        // Read the current FPCR value
        core::arch::asm!("mrs {}, fpcr", out(reg) old_fpcr);

        // Clear the rounding mode bits (bits 22-24)
        new_fpcr = old_fpcr & !(0b111 << 22);

        // Set the new rounding mode based on the given mode
        new_fpcr |= match mode {
            Some(RoundingMode::RNE) => 0b00 << 22,
            Some(RoundingMode::RTZ) => 0b11 << 22,
            Some(RoundingMode::RDN) => 0b10 << 22,
            Some(RoundingMode::RUP) => 0b01 << 22,
            Some(RoundingMode::RMM) => 1 << 24,
            None => todo!(),
        };

        // Set the new FPCR value
        core::arch::asm!("msr fpcr, {}", in(reg) new_fpcr);

        // Perform the conversion from f32 to u64
        core::arch::asm!("FCVTZU {}, {}", in(reg) f32::from_bits(cpu.fload(rs1).to_bits() as u32), out(reg) result);

        // Restore the old FPCR value (to revert the rounding mode)
        core::arch::asm!("msr fpcr, {}", in(reg) old_fpcr);
    }
    cpu.set_fflags();
    cpu.store(rd, result as u64);
    Ok(cpu.pc + 4)
}