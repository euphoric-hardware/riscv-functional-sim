use std::{
    collections::BTreeMap,
    default,
    fmt::{write, Display},
    u64,
};

use crate::{
    bus::{self, Bus, Device},
    csrs::{self, Csrs},
    diff::{Diff, ExecutionState},
    trace,
};

use ::simple_soft_float;

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
        !self.reg_write.is_empty() || !self.freg_write.is_empty()
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
        Default::default()
    }

    pub fn load(&self, reg: u64) -> u64 {
        self.regs[reg as usize]
    }

    pub fn store(&mut self, reg: u64, value: u64) {
        if reg != 0 {
            self.regs[reg as usize] = value;
            self.commits.reg_write.insert(reg, value);
        }
    }

    pub fn fload(&self, reg: u64) -> simple_soft_float::F64 {
        self.fregs[reg as usize]
    }

    pub fn fstore(&mut self, reg: u64, value: simple_soft_float::F64) {
        if reg != 0 {
            self.fregs[reg as usize] = value;
            self.commits
                .freg_write
                .insert(reg, f64::from_bits(*value.bits()));
        }
    }

    pub fn privilege_mode(&self) -> PrivilegeMode {
        let mstatus = self.csrs.load_unchecked(Csrs::MSTATUS);
        let mpp = ((mstatus >> 11) & 0b11) as u8;
        PrivilegeMode::from(mpp)
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let mut bytes = [0; std::mem::size_of::<u32>()];
        bus.read(self.pc, &mut bytes).expect("invalid dram address");
        let insn = Insn::from_bytes(&bytes);
        let mut state = <ExecutionState as std::default::Default>::default();

        match self.execute_insn(insn, bus) {
            Ok(pc) => {
                log::info!(
                    "core   0: {} 0x{:016x} (0x{:08x})",
                    self.privilege_mode(),
                    self.pc,
                    insn.bits()
                );

                state.pc = self.pc;
                state.instruction = insn.bits() as u32;

                if self.commits.modified_regs() {
                    while let Some((reg, val)) = self.commits.reg_write.pop_first() {
                        log::info!(" {:<3} 0x{:016x}", REGISTER_NAMES[reg as usize], val);
                        state.register_updates.push((reg as u8, val));
                    }
                }
                if self.commits.is_load() {
                    while let Some((addr, _)) = self.commits.mem_read.pop_first() {
                        log::info!(" mem 0x{:016x}", addr);
                    }
                } else if self.commits.is_store() {
                    while let Some((addr, val)) = self.commits.mem_write.pop_first() {
                        log::info!(" mem 0x{:016x} {}", addr, val);
                        state.memory_writes.push((addr, u64::from(val)));
                    }
                }
                log::info!("\n");
                self.states.push(state);

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

#[derive(Debug)]
pub enum InsnType {
    R { rd: u8, rs1: u8, rs2: u8 },
    I { rd: u8, rs1: u8, imm: i64 },
    S { rs1: u8, rs2: u8, imm: i64 },
    B { rs1: u8, rs2: u8, offset: i64 },
    U { rd: u8, imm: i64 },
    J { rd: u8, offset: i64 },
    CR { rd_rs1: u8, rs2: u8 },
    CI { rd_rs1: u8, imm: i64 },
    CSS { rs2: u8, imm: u64 },
    CIW { rd: u8, imm: i64 },
    CL { rd: u8, rs1: u8, imm: i64 },
    CS { rs1: u8, rs2: u8, imm: i64 },
    CA { rd_rs1: u8, rs2: u8 },
    CB { rd_rs1: u8, imm: i64 },
    CJ { imm: i64 },

    CsrReg { rd: u8, csr: u16, rs1: u8 },
    CsrImm { rd: u8, csr: u16, imm: u8 },
    Privileged,
}

#[rustfmt::skip]
mod insn_type_macros {
    macro_rules! r_type { ($rd:expr, $rs1:expr, $rs2:expr) => { crate::cpu::InsnType::R { rd: $rd as u8, rs1: $rs1 as u8, rs2: $rs2 as u8 } }; }
    macro_rules! i_type { ($rd:expr, $rs1:expr, $imm:expr) => { crate::cpu::InsnType::I { rd: $rd as u8, rs1: $rs1 as u8, imm: $imm as i64 } }; }
    macro_rules! s_type { ($rs1:expr, $rs2:expr, $imm:expr) => { crate::cpu::InsnType::S { rs1: $rs1 as u8, rs2: $rs2 as u8, imm: $imm as i64 } }; }
    macro_rules! b_type { ($rs1:expr, $rs2:expr, $offset:expr) => { crate::cpu::InsnType::B { rs1: $rs1 as u8, rs2: $rs2 as u8, offset: $offset as i64 } }; }
    macro_rules! u_type { ($rd:expr, $imm:expr) => { crate::cpu::InsnType::U { rd: $rd as u8, imm: $imm as i64 } }; }
    macro_rules! j_type { ($rd:expr, $offset:expr) => { crate::cpu::InsnType::J { rd: $rd as u8, offset: $offset as i64 } }; }
    macro_rules! cr_type { ($rd_rs1:expr, $rs2: expr) => { crate::cpu::InsnType::CR { rd_rs1: $rd_rs1 as u8, rs2: $rs2 as u8 } }; }
    macro_rules! ci_type { ($rd_rs1:expr, $imm: expr) => { crate::cpu::InsnType::CI { rd_rs1: $rd_rs1 as u8, imm: $imm as i8 as i64 } }; }
    macro_rules! css_type { ($rs2:expr, $imm: expr) => { crate::cpu::InsnType::CSS { rs2: $rs2 as u8, imm: $imm as u8 as u64 } }; }
    macro_rules! ciw_type { ($rd:expr, $imm: expr) => { crate::cpu::InsnType::CIW { rd: $rd as u8, imm: $imm as i16 as i64 } }; }
    macro_rules! cl_type { ($rd:expr, $rs1:expr, $imm: expr) => { crate::cpu::InsnType::CL { rd: $rd as u8, rs1: $rs1 as u8, imm: $imm as i8 as i64 } }; }
    macro_rules! cs_type { ($rs1:expr, $rs2:expr, $imm: expr) => { crate::cpu::InsnType::CS { rs1: $rs1 as u8, rs2: $rs2 as u8, imm: $imm as i8 as i64 } }; }
    macro_rules! ca_type { ($rd_rs1:expr, $rs2:expr) => { crate::cpu::InsnType::CA { rd_rs1: $rd_rs1 as u8, rs2: $rs2 as u8} }; }
    macro_rules! cb_type { ($rd_rs1:expr, $imm: expr) => { crate::cpu::InsnType::CB { rd_rs1: $rd_rs1 as u8, imm: $imm as i8 as i64 } }; }
    macro_rules! cj_type { ($imm: expr) => { crate::cpu::InsnType::CJ { imm: $imm as i16 as i64 } }; }

    macro_rules! csr_reg_type { ($rd:expr, $csr:expr, $rs1:expr) => { crate::cpu::InsnType::CsrReg { rd: $rd as u8, csr: $csr as u16, rs1: $rs1 as u8 } }; }
    macro_rules! csr_imm_type { ($rd:expr, $csr:expr, $imm:expr) => { crate::cpu::InsnType::CsrImm { rd: $rd as u8, csr: $csr as u16, imm: $imm as u8 } }; }

    pub(crate) use csr_imm_type;
    pub(crate) use csr_reg_type;
    pub(crate) use i_type;
    pub(crate) use j_type;
    pub(crate) use r_type;
    pub(crate) use b_type;
    pub(crate) use s_type;
    pub(crate) use u_type;
    pub(crate) use cr_type;
    pub(crate) use ci_type;
    pub(crate) use css_type;
    pub(crate) use ciw_type;
    pub(crate) use cl_type;
    pub(crate) use cs_type;
    pub(crate) use ca_type;
    pub(crate) use cb_type;
    pub(crate) use cj_type;

}

pub(crate) use insn_type_macros::*;
use log::info;
use simple_soft_float::RoundingMode;

impl Display for InsnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InsnType::*;

        #[inline]
        fn r(reg: &u8) -> &'static str {
            REGISTER_NAMES[*reg as usize]
        }

        match self {
            R { rd, rs1, rs2 } => write!(f, "{}, {}, {}", r(rd), r(rs1), r(rs2)),
            I { rd, rs1, imm } => write!(f, "{}, {}, {:#x}", r(rd), r(rs1), imm),
            S { rs1, rs2, imm } => write!(f, "{}, {}({})", r(rs2), imm, r(rs1)),
            B { rs1, rs2, offset } => write!(f, "{}, {}, {}", r(rs1), r(rs2), offset),
            U { rd, imm } => write!(f, "{}, {:#x}", r(rd), imm),
            J { rd, offset } => write!(
                f,
                "{}, pc {} {}",
                r(rd),
                if *offset >= 0 { '+' } else { '-' },
                offset.abs()
            ),
            CR { rd_rs1, rs2 } => write!(f, "{}, 0({})", r(rs2), r(rd_rs1)),
            CI { rd_rs1, imm } => write!(f, "{}, {}", r(rd_rs1), imm),
            CSS { rs2, imm } => write!(f, "{}, {}(sp)", r(rs2), imm),
            CIW { rd, imm } => write!(f, "{}, sp, {}", r(rd), imm),
            CL { rd, rs1, imm } => write!(f, "{}, {}({})", r(rd), r(rs1), imm),
            CS { rs1, rs2, imm } => write!(f, "{}, {}({})", r(rs2), imm, r(rs2)),
            CA { rd_rs1, rs2 } => write!(f, "{}, {}, {}", r(rd_rs1), r(rd_rs1), r(rs2)),
            CB { rd_rs1, imm } => write!(f, "{}, {}", r(rd_rs1), imm),
            CJ { imm } => write!(f, "{}", imm),
            CsrReg { rd, csr, rs1 } => write!(f, "{}, {:#x}, {}", r(rd), csr, r(rs1)),
            CsrImm { rd, csr, imm } => write!(f, "{}, {:#x}, {}", r(rd), csr, imm),
            Privileged => write!(f, ""),
        }
    }
}
