pub struct FRegFile {
    registers: [f64; 32], // 32 floating-point registers, default to 64-bit doubles
}

impl FRegFile {
    pub fn new() -> Self {
        Self {
            registers: [0.0; 32],
        }
    }

    pub fn write_register(&mut self, index: usize, value: f64) {
        self.registers[index] = value;
    }

    pub fn read_register(&self, index: usize) -> f64 {
        self.registers[index]
    }
}