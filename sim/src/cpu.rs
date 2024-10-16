#[derive(Default)]
pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub dram: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct Insn(pub u64);

impl Insn {
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
