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

    pub fn rs1(&self) -> u64 {
        self.bit_range(15, 5)
    }

    pub fn rs2(&self) -> u64 {
        self.bit_range(20, 5)
    }

    pub fn rd(&self) -> u64 {
        self.bit_range(7, 5)
    }

    pub fn u_imm(&self) -> u64 {
        self.bit_range_sign_extended(12, 20)
    }

    pub fn s_imm(&self) -> u64 {
        (self.bit_range_sign_extended(25, 7) << 7) | self.bit_range(0, 5)
    }

    pub fn imm12(&self) -> u64 {
        todo!()
    }

    pub fn imm12hi(&self) -> u64 {
        todo!()
    }

    pub fn imm12lo(&self) -> u64 {
        todo!()
    }

    pub fn bimm12hi(&self) -> u64 {
        todo!()
    }

    pub fn bimm12lo(&self) -> u64 {
        todo!()
    }

    pub fn shamtd(&self) -> u64 {
        todo!()
    }

    pub fn shamtw(&self) -> u64 {
        todo!()
    }

    pub fn jimm20(&self) -> u64 {
        todo!()
    }

    pub fn imm20(&self) -> u64 {
        todo!()
    }

    pub fn fm(&self) -> u64 {
        todo!()
    }

    pub fn pred(&self) -> u64 {
        todo!()
    }

    pub fn succ(&self) -> u64 {
        todo!()
    }
}
