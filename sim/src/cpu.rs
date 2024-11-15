#[derive(Default)]
pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub dram: Vec<u8>,
    pub fregisters: [f64; 32],
}

#[derive(Clone, Copy)]
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

    fn bit_range_sign_extended(&self, offset: u8, length: u8) -> u64 {
        ((self.bits() as i64) << (64 - offset - length) >> (64 - length)) as u64
    }
}

pub struct Instruction(u32);

impl Instruction {
    pub fn opcode(&self) -> u8 {
        (self.0 & 0x7F) as u8 // Bits [6:0]
    }

    pub fn rd(&self) -> u8 {
        ((self.0 >> 7) & 0x1F) as u8 // Bits [11:7]
    }

    pub fn funct3(&self) -> u8 {
        ((self.0 >> 12) & 0x07) as u8 // Bits [14:12]
    }

    pub fn rs1(&self) -> u8 {
        ((self.0 >> 15) & 0x1F) as u8 // Bits [19:15]
    }

    pub fn rs2(&self) -> u8 {
        ((self.0 >> 20) & 0x1F) as u8 // Bits [24:20]
    }

    pub fn funct7(&self) -> u8 {
        ((self.0 >> 25) & 0x7F) as u8 // Bits [31:25]
    }

    pub fn rs3(&self) -> u8 {
        ((self.0 >> 27) & 0x1F) as u8 // Bits [31:27]
    }
}
