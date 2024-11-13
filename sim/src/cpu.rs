use crate::{
    bus::{self, Bus, Device},
    csrs::Csrs,
};

#[derive(Debug, Default)]
pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub csrs: Csrs,
}

pub enum Error {
    UnknownCsr,
    UnknownInsn,
    BusError(bus::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Cpu {
    pub fn new() -> Cpu {
        Default::default()
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let mut bytes = [0; std::mem::size_of::<u32>()];
        bus.read(self.pc, &mut bytes).expect("invalid dram address");
        let insn = Insn::from_bytes(&bytes);

        if let Ok(pc) = self.execute_insn(insn, bus) {
            self.pc = pc;
        } else {
            self.csrs.store_unchecked(Csrs::MEPC, self.pc);
            self.csrs.store_unchecked(Csrs::MCAUSE, 0x2); // invalid insn
            self.pc = self.csrs.load_unchecked(Csrs::MTVEC);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Insn(pub u64);

impl Insn {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64)
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
