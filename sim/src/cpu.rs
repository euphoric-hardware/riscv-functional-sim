use std::fmt::{write, Display};

use crate::{
    bus::{self, Bus, Device},
    csrs::Csrs,
    trace,
};

#[derive(Debug, Default)]
pub struct Regfile([u64; 32]);

impl Regfile {
    pub fn load(&self, reg: u64) -> u64 {
        self.0[reg as usize]
    }

    pub fn store(&mut self, reg: u64, value: u64) {
        if reg != 0 {
            self.0[reg as usize] = value;
        }
    }
}

#[derive(Debug, Default)]
pub struct Cpu {
    pub regs: Regfile,
    pub pc: u64,
    pub csrs: Csrs,
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

pub type Result<T> = std::result::Result<T, Exception>;

impl Cpu {
    pub fn new() -> Cpu {
        Default::default()
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let mut bytes = [0; std::mem::size_of::<u32>()];
        bus.read(self.pc, &mut bytes).expect("invalid dram address");
        let insn = Insn::from_bytes(&bytes);

        match self.execute_insn(insn, bus) {
            Ok(pc) => self.pc = pc,
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
            (value as i64) | !((1 << length) - 1) as i64
        } else {
            value as i64
        }
    }
}

// FOR TRACING PURPOSES

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

#[derive(Debug)]
pub enum InsnType {
    R { rd: u8, rs1: u8, rs2: u8 },
    I { rd: u8, rs1: u8, imm: i64 },
    S { rs1: u8, rs2: u8, imm: i64 },
    B { rs1: u8, rs2: u8, offset: i64 },
    U { rd: u8, imm: i64 },
    J { rd: u8, offset: i64 },
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
}

pub(crate) use insn_type_macros::*;

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
            S { rs1, rs2, imm } => write!(f, "{}, {}({})", r(rs1), imm, r(rs2)),
            B { rs1, rs2, offset } => write!(f, "{}, {}, {}", r(rs1), r(rs2), offset),
            U { rd, imm } => write!(f, "{}, {:#x}", r(rd), imm),
            J { rd, offset } => write!(
                f,
                "{}, pc {} {}",
                r(rd),
                if *offset >= 0 { '+' } else { '-' },
                offset.abs()
            ),
            CsrReg { rd, csr, rs1 } => write!(f, "{}, {:#x}, {}", r(rd), csr, r(rs1)),
            CsrImm { rd, csr, imm } => write!(f, "{}, {:#x}, {}", r(rd), csr, imm),
            Privileged => write!(f, ""),
        }
    }
}
