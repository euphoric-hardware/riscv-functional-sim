use simple_soft_float::F64;

use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn, RoundingMode},
};

pub fn fcvt_s_lu_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let input: u64 = cpu.load(rs1);
    let mode = Insn::get_rounding_mode(cpu, rm);
    cpu.update_hardware_fp_flags();

    let result: f32;
    #[cfg(target_arch = "aarch64")]
    {
        unsafe {
            // Set the FPCR based on rounding mode
            let mode_bits = match mode {
                Some(RoundingMode::RNE) => 0b00,
                Some(RoundingMode::RUP) => 0b01,
                Some(RoundingMode::RDN) => 0b10,
                Some(RoundingMode::RTZ) => 0b11,
                Some(RoundingMode::RMM) => 0b11,
                None => todo!(),
            };

            let mut fpcr: u64;
            core::arch::asm!("mrs {0}, fpcr", out(reg) fpcr);
            fpcr = (fpcr & !(0b11 << 22)) | ((mode_bits as u64) << 22);
            core::arch::asm!("msr fpcr, {0}", in(reg) fpcr);

            core::arch::asm!(
                "scvtf d0, {input}",
                "fmov {output}, d0",
                input = in(reg) input,
                output = out(reg) result,
            );
        }
    }

    cpu.set_fflags();
    cpu.fstore(rd, Insn::f32_to_f64_raw(result));
    Ok(cpu.pc + 4)
}
