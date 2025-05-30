use std::{
    arch::asm,
    collections::{BTreeMap, HashMap},
    default,
    fmt::{write, Display},
    hash::Hash,
};

use crate::{
    branch_hints::{likely, unlikely},
    bus::{Bus, Device},
    csrs::{self, Csrs},
    diff::ExecutionState,
    uop_cache::uop_cache::UopCacheEntry,
    DIFF, LOG,
};

use ::simple_soft_float;
use log::info;

#[derive(Debug, Default, Clone)]
pub enum MemData {
    DoubleWord(u64),
    Word(u32),
    HalfWord(u16),
    Byte(u8),
    #[default]
    Empty,
}

impl MemData {
    // SAFETY: slice must be 8, 4, 2, or 1 byte long
    pub fn from_le_bytes(buf: &[u8]) -> Self {
        use MemData::*;
        match buf.len() {
            8 => DoubleWord(u64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ])),
            4 => Word(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])),
            2 => HalfWord(u16::from_le_bytes([buf[0], buf[1]])),
            1 => Byte(buf[0]),
            _ => unreachable!(),
        }
    }
}

impl From<MemData> for u64 {
    fn from(data: MemData) -> Self {
        match data {
            MemData::DoubleWord(val) => val,
            MemData::Word(val) => val as u64,
            MemData::HalfWord(val) => val as u64,
            MemData::Byte(val) => val as u64,
            MemData::Empty => 0,
        }
    }
}

impl Display for MemData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MemData::*;
        match self {
            DoubleWord(dw) => write!(f, "0x{:016x}", dw),
            Word(w) => write!(f, "0x{:08x}", w),
            HalfWord(hw) => write!(f, "0x{:04x}", hw),
            Byte(b) => write!(f, "0x{:02x}", b),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Commits {
    // reg, data
    pub reg_write: BTreeMap<u64, u64>,
    pub freg_write: BTreeMap<u64, f64>,
    // addr, data
    pub mem_write: BTreeMap<u64, MemData>,
    pub mem_read: BTreeMap<u64, MemData>,
}

impl Commits {
    pub fn is_load(&self) -> bool {
        !self.mem_read.is_empty()
    }

    pub fn is_store(&self) -> bool {
        !self.mem_write.is_empty()
    }

    pub fn modified_regs(&self) -> bool {
        !self.reg_write.is_empty()
    }

    pub fn modified_fregs(&self) -> bool {
        !self.freg_write.is_empty()
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PrivilegeMode {
    User = 0,
    Supervisor = 1,
    Machine = 3,
}

impl Display for PrivilegeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl From<u8> for PrivilegeMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::User,
            1 => Self::Supervisor,
            3 => Self::Machine,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Cpu {
    pub regs: [u64; 32],
    pub fregs: [f64; 32],
    pub pc: u64,
    pub csrs: Csrs,
    pub uop_cache: Vec<Option<UopCacheEntry>>,
    pub uop_base: u64,   // start_pc
    pub uop_stride: u64, // 2
    pub commits: Commits,
    pub states: Vec<ExecutionState>,
    pub cache_hits: u64,
}

#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAccessMisaligned = 4,
    LoadAccessFault = 5,
    StoreAMOAddressMisaligned = 6,
    StoreAMOAccessFault = 7,
    EnvironmentCallFromUMode = 8,
    EnvironmentCallFromSMode = 9,
    EnvironmentCallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StoreAMOPageFault = 15,
}

#[derive(Debug)]
pub enum FpOperation {
    Add,
    Sub,
    Mul,
    Div,
    Sqrt,
    Convert,
    Fmadd,
    Fmsub,
    Fnmadd,
    Fnmsub,
}

#[derive(Debug, Clone, Copy)]
pub enum RoundingMode {
    RNE, // Round to Nearest, ties to Even
    RTZ, // Round Toward Zero
    RDN, // Round Down (−∞)
    RUP, // Round Up (+∞)
    RMM, // Round to Nearest, ties to Max Magnitude (ARM only)
}

pub type Result<T> = std::result::Result<T, Exception>;

impl Cpu {
    pub fn new() -> Cpu {
        let cpu: Cpu = Default::default();
        cpu
    }

    pub fn load_uop_cache(&mut self, bus: &mut Bus, start_pc: u64, end_pc: u64) {
        self.uop_base = start_pc;
        self.uop_stride = 2;
        let size = ((end_pc - start_pc) / self.uop_stride) as usize + 1;
        self.uop_cache = vec![None; size];

        let mut i: u64 = start_pc;
        while i < end_pc {
            let mut bytes = [0; std::mem::size_of::<u32>()];
            bus.read(i, &mut bytes).expect("invalid dram address");
            let insn = Insn::from_bytes(&bytes);
            let entry = UopCacheEntry::new(insn);
            if let Some(entry) = entry {
                let index = ((i - self.uop_base) / self.uop_stride) as usize;
                self.uop_cache[index] = Some(entry);
            }

            i += 2;
            if insn.bits() & 0b11 == 0b11 {
                i += 2;
            }
        }
    }

    #[inline(always)]
    pub fn get_uop(&self, addr: u64) -> Option<&UopCacheEntry> {
        let index = ((addr - self.uop_base) / self.uop_stride) as usize;
        self.uop_cache.get(index).and_then(|e| e.as_ref())
    }

    #[inline(always)]
    pub fn load(&self, reg: u64) -> u64 {
        unsafe { *self.regs.get_unchecked(reg as usize) }
    }

    #[inline(always)]
    pub fn store(&mut self, reg: u64, value: u64) {
        if likely(reg != 0) {
            unsafe {
                *self.regs.get_unchecked_mut(reg as usize) = value;
            }
            #[cfg(debug_assertions)]
            if unlikely(*DIFF.get().unwrap()) {
                self.commits.reg_write.insert(reg, value);
            }
        }
    }

    #[inline(always)]
    pub fn fload(&self, reg: u64) -> f64 {
        unsafe { *self.fregs.get_unchecked(reg as usize) }
    }

    #[inline(always)]
    pub fn fstore(&mut self, reg: u64, value: f64) {
        unsafe {
            *self.fregs.get_unchecked_mut(reg as usize) = value;
        }
        #[cfg(debug_assertions)]
        if unlikely(*DIFF.get().unwrap()) {
            self.commits.freg_write.insert(reg, value);
        }
    }

    pub fn privilege_mode(&self) -> PrivilegeMode {
        let mstatus = self.csrs.load_unchecked(Csrs::MSTATUS);
        let mpp = ((mstatus >> 11) & 0b11) as u8;
        PrivilegeMode::from(mpp)
    }

    pub fn get_hardware_fp_flags() -> u32 {
        #[cfg(target_arch = "x86_64")]
        {
            // read fnstsw or MXCSR, decode flags
            0
        }

        #[cfg(target_arch = "aarch64")]
        {
            let fpsr: u32;
            unsafe {
                asm!(
                    "mrs {0}, fpsr",
                    out(reg) fpsr
                );
            }

            // Extract the relevant bits and reorder
            let nv = (fpsr >> 0) & 1;
            let dz = (fpsr >> 1) & 1;
            let of = (fpsr >> 2) & 1;
            let uf = (fpsr >> 3) & 1;
            let nx = (fpsr >> 4) & 1;

            // Reassemble into desired order: nv (4), dz (3), of (2), uf (1), nx (0)
            ((nv << 4) | (dz << 3) | (of << 2) | (uf << 1) | nx) as u32
        }

        // fallback or unsupported arch
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            vec![]
        }
    }

    pub fn update_hardware_fp_flags(&self) {
        let mask = self
            .csrs
            .load(Csrs::FFLAGS)
            .expect("Error reading from FFLAGS!");
        #[cfg(target_arch = "aarch64")]
        {
            let nv = (mask >> 4) & 1; // FPSR bit 0
            let dz = (mask >> 3) & 1; // FPSR bit 1
            let of = (mask >> 2) & 1; // FPSR bit 2
            let uf = (mask >> 1) & 1; // FPSR bit 3
            let nx = (mask >> 0) & 1; // FPSR bit 4

            let fpsr_val: u32 = ((nx << 4) | (uf << 3) | (of << 2) | (dz << 1) | nv) as u32;

            unsafe {
                asm!(
                    "msr fpsr, {0}",
                    in(reg) fpsr_val,
                    options(nostack, nomem)
                );
            }
        }
    }

    pub fn print_fpsr_exceptions() {
        let fpsr: u32;
        unsafe {
            asm!(
                "mrs {0}, fpsr",
                out(reg) fpsr
            );
        }

        let flags = [
            ("IOC (Invalid Operation)", (fpsr >> 0) & 1),
            ("DZC (Divide by Zero)", (fpsr >> 1) & 1),
            ("OFC (Overflow)", (fpsr >> 2) & 1),
            ("UFC (Underflow)", (fpsr >> 3) & 1),
            ("IXC (Inexact)", (fpsr >> 4) & 1),
            ("IDC (Input Denormal)", (fpsr >> 7) & 1),
        ];

        println!("Floating-Point Exception Flags:");
        println!("Raw FPSR: {:#034b}", fpsr);
        for (label, state) in flags {
            println!("  {:<25}: {}", label, state);
        }
    }

    pub fn clear_fpsr_exceptions() {
        unsafe {
            asm!(
                "msr fpsr, xzr", // Write zero to FPSR — clears all flags
                options(nostack, nomem)
            );
        }
    }

    pub fn set_fflags(&mut self) {
        self.csrs
            .store(Csrs::FFLAGS, Self::get_hardware_fp_flags() as u64);
    }

    #[inline(always)]
    pub fn step(&mut self, bus: &mut Bus) {
        #[cfg(debug_assertions)]
        let log = *LOG.get().expect("invalid LOG global variable");

        #[cfg(debug_assertions)]
        let mut state = {
            if *DIFF.get().unwrap() {
                ExecutionState {
                    pc: self.pc,
                    ..Default::default()
                }
            } else {
                Default::default()
            }
        };

        let insn_bits = self.get_uop(self.pc).map_or(0, |entry| entry.insn_bits);

        match self.execute_insn(bus) {
            Ok(new_pc) => {
                // diffing and logging
                #[cfg(debug_assertions)]
                {
                    if log {
                        info!(
                            "core   0: {} 0x{:016x} (0x{:08x})",
                            self.privilege_mode(),
                            self.pc,
                            insn_bits
                        );
                    }

                    if self.commits.modified_regs() {
                        for (reg, val) in self.commits.reg_write.iter() {
                            if log {
                                info!(" {:<3} 0x{:016x}", REGISTER_NAMES[*reg as usize], *val);
                            }
                            if *DIFF.get().unwrap() {
                                state.register_updates.push((*reg as u8, *val));
                            }
                        }
                        self.commits.reg_write.clear();
                    }

                    if self.commits.modified_fregs() {
                        for (reg, val) in self.commits.freg_write.iter() {
                            if log {
                                info!(" f{:<3} 0x{:016x}", *reg as usize, val.to_bits());
                            }
                            if *DIFF.get().unwrap() {
                                state.fregister_updates.push((*reg as u8, val.to_bits()));
                            }
                        }
                        self.commits.freg_write.clear();
                    }

                    if self.commits.is_load() {
                        for (addr, _) in self.commits.mem_read.iter() {
                            if log {
                                info!(" mem 0x{:016x}", *addr);
                            }
                        }
                        self.commits.mem_read.clear();
                    } else if self.commits.is_store() {
                        for (addr, val) in self.commits.mem_write.iter() {
                            if log {
                                info!(" mem 0x{:016x} {}", *addr, val);
                            }

                            if *DIFF.get().unwrap() {
                                state.memory_writes.push((*addr, u64::from(val.clone())));
                            }
                        }
                        self.commits.mem_write.clear();
                    }

                    if log {
                        info!("\n");
                    }

                    if *DIFF.get().unwrap() {
                        state.instruction = insn_bits as u32;
                        self.states.push(state);
                    }
                }

                self.pc = new_pc;
                unsafe {
                    *self
                        .csrs
                        .regs
                        .get_unchecked_mut(csrs::Csrs::MCYCLE as usize) = self
                        .csrs
                        .regs
                        .get_unchecked(csrs::Csrs::MCYCLE as usize)
                        .wrapping_add(1);
                }
            }
            Err(e) => unsafe {
                self.csrs.store_unchecked(Csrs::MCAUSE, e as u64);
                self.csrs.store_unchecked(Csrs::MEPC, self.pc);
                self.pc = self.csrs.load_unchecked(Csrs::MTVEC);
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Insn(pub u64);

impl Insn {
    pub fn from_bits(bits: u32) -> Self {
        Self(bits as u64)
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64)
    }

    pub fn bits(&self) -> u64 {
        self.0
    }

    pub fn bit_range(&self, offset: u8, length: u8) -> u64 {
        (self.bits() >> offset) & ((1 << length) - 1)
    }

    pub fn bit_range_sign_extended(&self, offset: u8, length: u8) -> u64 {
        ((self.bits() as i64) << (64 - offset - length) >> (64 - length)) as u64
    }

    pub fn sign_extend(value: u64, length: u8) -> i64 {
        let sign_bit = 1u64 << (length - 1);
        if value & sign_bit != 0 {
            (value as i64) | !((1u64 << length) - 1) as i64
        } else {
            value as i64
        }
    }

    pub fn classify_f32(val: f32) -> u32 {
        use std::num::FpCategory;

        match val.classify() {
            FpCategory::Infinite => {
                if val.is_sign_negative() {
                    1_u32 << 0 // -inf
                } else {
                    1_u32 << 7 // +inf
                }
            }
            FpCategory::Normal => {
                if val.is_sign_negative() {
                    1_u32 << 1 // negative normal
                } else {
                    1_u32 << 6 // positive normal
                }
            }
            FpCategory::Subnormal => {
                if val.is_sign_negative() {
                    1_u32 << 2 // negative subnormal
                } else {
                    1_u32 << 5 // positive subnormal
                }
            }
            FpCategory::Zero => {
                if val.is_sign_negative() {
                    1_u32 << 3 // negative zero
                } else {
                    1_u32 << 4 // positive zero
                }
            }
            FpCategory::Nan => {
                let bits = val.to_bits();
                let quiet_bit = 1_u32 << 22;
                if bits & quiet_bit == 0 {
                    1_u32 << 8 // signaling NaN
                } else {
                    1_u32 << 9 // quiet NaN
                }
            }
        }
    }

    pub fn classify_f64(val: f64) -> u32 {
        use std::num::FpCategory;

        match val.classify() {
            FpCategory::Infinite => {
                if val.is_sign_negative() {
                    1_u32 << 0 // -inf
                } else {
                    1_u32 << 7 // +inf
                }
            }
            FpCategory::Normal => {
                if val.is_sign_negative() {
                    1_u32 << 1 // negative normal
                } else {
                    1_u32 << 6 // positive normal
                }
            }
            FpCategory::Subnormal => {
                if val.is_sign_negative() {
                    1_u32 << 2 // negative subnormal
                } else {
                    1_u32 << 5 // positive subnormal
                }
            }
            FpCategory::Zero => {
                if val.is_sign_negative() {
                    1_u32 << 3 // negative zero
                } else {
                    1_u32 << 4 // positive zero
                }
            }
            FpCategory::Nan => {
                let bits = val.to_bits();
                let quiet_bit = 1_u64 << 51;
                if bits & quiet_bit == 0 {
                    1_u32 << 8 // signaling NaN
                } else {
                    1_u32 << 9 // quiet NaN
                }
            }
        }
    }

    pub fn is_signaling_nan_f64(val: f64) -> bool {
        let bits = val.to_bits();
        let exponent = (bits >> 52) & 0x7FF;
        let fraction = bits & 0x000F_FFFF_FFFF_FFFF;
        let quiet_bit = 1 << 51;

        exponent == 0x7FF && fraction != 0 && (fraction & quiet_bit == 0)
    }

    pub fn is_signaling_nan_f32(val: f32) -> bool {
        let bits = val.to_bits();
        let exponent = (bits >> 23) & 0xFF;
        let fraction = bits & 0x007F_FFFF;
        let quiet_bit = 1 << 22;

        exponent == 0xFF && fraction != 0 && (fraction & quiet_bit == 0)
    }

    pub fn f32_to_f64_raw(val: f32) -> f64 {
        f64::from_bits(0xffffffff00000000 | val.to_bits() as u64)
    }

    pub fn get_rounding_mode(cpu: &mut Cpu, rm: u64) -> Option<RoundingMode> {
        match rm {
            0 => Some(RoundingMode::RNE),
            1 => Some(RoundingMode::RTZ),
            2 => Some(RoundingMode::RDN),
            3 => Some(RoundingMode::RUP),
            4 => Some(RoundingMode::RMM),
            7 => Self::get_rounding_mode(
                cpu,
                cpu.csrs.load(Csrs::FRM).expect("invalid rounding mode"),
            ),
            _ => None,
        }
    }
}

// FOR TRACING PURPOSES

static REGISTER_NAMES: [&str; 32] = [
    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14",
    "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27",
    "x28", "x29", "x30", "x31",
];
