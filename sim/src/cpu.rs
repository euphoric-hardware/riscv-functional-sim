use std::{
    collections::{BTreeMap, HashMap},
    default,
    fmt::{write, Display},
    hash::Hash,
    u64,
};

use crate::{
    ahash::AHashMap,
    bus::{Bus, Device},
    csrs::Csrs,
    diff::ExecutionState,
    uop_cache::uop_cache::UopCacheEntry,
    DIFF,
};

use ::simple_soft_float;
use simple_soft_float::RoundingMode;

use log::info;

#[derive(Debug, Default)]
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
    pub freg_write: BTreeMap<u64, u64>,
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
    pub fregs: [simple_soft_float::F64; 32],
    pub pc: u64,
    pub csrs: Csrs,
    pub uop_cache: AHashMap<u64, UopCacheEntry>,
    pub diff: bool,
    pub commits: Commits,
    pub states: Vec<ExecutionState>,
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

pub type Result<T> = std::result::Result<T, Exception>;

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu: Cpu = Default::default();
        cpu.diff = *(DIFF.get().expect("invalid DIFF global variable"));
        cpu
    }

    pub fn load_uop_cache(&mut self, bus: &mut Bus, start_pc: u64, end_pc: u64) {
        let mut i: u64 = start_pc;
        while (i < end_pc) {
            let mut bytes = [0; std::mem::size_of::<u32>()];
            bus.read(i, &mut bytes).expect("invalid dram address");
            let insn = Insn::from_bytes(&bytes);
            let cache_index = i;
            i += 2;

            let entry = UopCacheEntry::new(insn);
            if let Some(entry) = entry {
                self.uop_cache.insert(cache_index, entry);
            }

            if insn.bits() & 0b11 == 0b11 {
                i += 2; // regular length instructions
            }
        }
    }

    pub fn load(&self, reg: u64) -> u64 {
        self.regs[reg as usize]
    }

    pub fn store(&mut self, reg: u64, value: u64) {
        if reg != 0 {
            self.regs[reg as usize] = value;
            if self.diff {
                self.commits.reg_write.insert(reg, value);
            }
        }
    }

    pub fn fload(&self, reg: u64) -> simple_soft_float::F64 {
        self.fregs[reg as usize]
    }

    pub fn fstore(&mut self, reg: u64, value: simple_soft_float::F64) {
        self.fregs[reg as usize] = value;
        if self.diff {
            self.commits.freg_write.insert(reg, *value.bits());
        }
    }

    pub fn privilege_mode(&self) -> PrivilegeMode {
        let mstatus = self.csrs.load_unchecked(Csrs::MSTATUS);
        let mpp = ((mstatus >> 11) & 0b11) as u8;
        PrivilegeMode::from(mpp)
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let mut state = <ExecutionState as std::default::Default>::default();
        let mut insn_bits = 0;
        if let Some(entry) = self.uop_cache.get(&self.pc) {
            insn_bits = entry.insn_bits;
        }
        match self.execute_insn(bus) {
            Ok(pc) => {
                let diff: bool = *DIFF.get().expect("invalid DIFF global variable");
                if diff == true {
                    info!(
                        "core   0: {} 0x{:016x} (0x{:08x})",
                        self.privilege_mode(),
                        self.pc,
                        insn_bits
                    );

                    state.pc = self.pc;
                    state.instruction = insn_bits as u32; // FIXME - figure out how to get instruction

                    if self.commits.modified_regs() {
                        while let Some((reg, val)) = self.commits.reg_write.pop_first() {
                            info!(" {:<3} 0x{:016x}", REGISTER_NAMES[reg as usize], val);
                            state.register_updates.push((reg as u8, val));
                        }
                    }
                    if self.commits.modified_fregs() {
                        while let Some((reg, val)) = self.commits.freg_write.pop_first() {
                            info!(" f{:<3} 0x{:016x}", reg as usize, val);
                            state.fregister_updates.push((reg as u8, val));
                        }
                    }
                    if self.commits.is_load() {
                        while let Some((addr, _)) = self.commits.mem_read.pop_first() {
                            info!(" mem 0x{:016x}", addr);
                        }
                    } else if self.commits.is_store() {
                        while let Some((addr, val)) = self.commits.mem_write.pop_first() {
                            info!(" mem 0x{:016x} {}", addr, val);
                            state.memory_writes.push((addr, u64::from(val)));
                        }
                    }
                    info!("\n");
                    self.states.push(state);
                }

                self.pc = pc;
                self.csrs.store(0xB00, self.csrs.load_unchecked(0xB00) + 1);
            }
            Err(e) => {
                self.csrs.store_unchecked(Csrs::MCAUSE, e as u64);
                self.csrs.store_unchecked(Csrs::MEPC, self.pc);
                self.pc = self.csrs.load_unchecked(Csrs::MTVEC);
            }
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

    pub fn f32_to_f64_preserve_all(value: f32) -> f64 {
        let bits = value.to_bits(); // Get raw bit pattern
        let sign = ((bits >> 31) as u64) << 63; // Extract sign bit and shift to f64 position
        let exponent = ((bits >> 23) & 0xFF) as u64; // Extract exponent
        let fraction = (bits & 0x007F_FFFF) as u64; // Extract fraction

        if exponent == 0xFF {
            // Handle NaNs and Infinities
            let new_fraction = fraction << 29; // Align mantissa
            let new_exponent = 0x7FF; // Max exponent for f64
            return f64::from_bits(sign | (new_exponent << 52) | new_fraction);
        } else if exponent == 0 {
            // Subnormal case (denormal numbers)
            let new_fraction = fraction << 29;
            return f64::from_bits(sign | new_fraction);
        }

        // Normalized numbers
        let new_exponent = (exponent as u64 + 896) << 52; // Adjust exponent bias from f32 (127) to f64 (1023)
        let new_fraction = fraction << 29; // Align fraction to f64

        f64::from_bits(sign | new_exponent | new_fraction)
    }

    pub fn f64_to_f32_preserve_all(value: f64) -> f32 {
        let bits = value.to_bits();
        let sign = ((bits >> 63) as u32) << 31; // Extract sign bit for f32
        let mut exponent = ((bits >> 52) & 0x7FF) as u32; // Extract exponent
        let mut fraction = ((bits & 0x000F_FFFF_FFFF_FFFF) >> 29) as u32; // Extract 23-bit fraction

        if exponent == 0x7FF {
            // Handle NaNs and Infinities
            let new_exponent = 0xFF;
            return f32::from_bits(sign | (new_exponent << 23) | fraction);
        } else if exponent == 0 {
            // Subnormal case (denormal numbers)
            return f32::from_bits(sign | fraction);
        }

        // Handle rounding to nearest even
        let round_bit = (bits >> 28) & 1; // First dropped bit
        let sticky_bits = bits & 0x0FFFFFFF; // Remaining dropped bits
        if round_bit == 1 && (sticky_bits != 0 || (fraction & 1) == 1) {
            fraction += 1;
        }

        // Handle possible mantissa overflow due to rounding
        if fraction >> 23 != 0 {
            fraction = 0; // Mantissa overflow -> increment exponent
            exponent += 1;
        }

        // Normalized numbers: Adjust exponent bias from f64 (1023) to f32 (127)
        let new_exponent = ((exponent as i32 - 1023 + 127) as u32) << 23;

        f32::from_bits(sign | new_exponent | fraction)
    }

    pub fn softfloat_flags_from_riscv_flags(cpu: &mut Cpu) -> simple_soft_float::StatusFlags {
        let riscv_flags = cpu.csrs.load_unchecked(Csrs::FFLAGS) as u32;
        let mask = 0b11111; // Mask to get the first 5 bits
        let relevant_bits = riscv_flags & mask; // Extract the first 5 bits

        let mut reversed_bits = 0u32;

        // Reverse the bits one by one
        for i in 0..5 {
            reversed_bits <<= 1; // Shift left to make space for the next bit
            reversed_bits |= (relevant_bits >> i) & 1; // Take the bit and append to reversed_bits
        }
        return simple_soft_float::StatusFlags::from_bits(reversed_bits)
            .expect("invalid bits received!");
    }

    pub fn riscv_flags_from_softfloat_flags(
        cpu: &mut Cpu,
        softfloat_status: simple_soft_float::StatusFlags,
    ) {
        let softfloat_flags = softfloat_status.bits();
        let mask = 0b11111; // Mask to get the first 5 bits
        let relevant_bits = softfloat_flags & mask; // Extract the first 5 bits

        let mut reversed_bits = 0u32;

        // Reverse the bits one by one
        for i in 0..5 {
            reversed_bits <<= 1; // Shift left to make space for the next bit
            reversed_bits |= (relevant_bits >> i) & 1; // Take the bit and append to reversed_bits
        }
        cpu.csrs.store(Csrs::FFLAGS, reversed_bits as u64);
    }

    pub fn softfloat_round_from_riscv_rm(rm: u64) -> RoundingMode {
        match rm {
            0b000 => RoundingMode::TiesToEven,
            0b001 => RoundingMode::TowardZero,
            0b010 => RoundingMode::TowardNegative,
            0b011 => RoundingMode::TowardPositive,
            0b100 => RoundingMode::TiesToAway,
            default => RoundingMode::TiesToAway,
        }
    }
}

// FOR TRACING PURPOSES

static REGISTER_NAMES: [&str; 32] = [
    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14",
    "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27",
    "x28", "x29", "x30", "x31",
];
