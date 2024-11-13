use crate::cpu::Result;

pub const NUM_CSRS: usize = 4096;

pub struct Csrs {
    pub regs: [u64; NUM_CSRS],
}

impl Default for Csrs {
    fn default() -> Self {
        Self {
            regs: [0; NUM_CSRS],
        }
    }
}

impl std::fmt::Debug for Csrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.regs)
    }
}

impl Csrs {
    // SAFETY: caller must ensure valid CSR adderss
    pub fn load_unchecked(&self, address: u64) -> u64 {
        self.regs[address as usize]
    }

    // SAFETY: caller must ensure valid CSR adderss
    pub fn store_unchecked(&mut self, address: u64, value: u64) {
        self.regs[address as usize] = value;
    }
}
