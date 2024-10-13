pub struct Memory {
    data: Vec<u8>
}

impl Memory {
    pub fn new_memory(size: usize) -> Self {
        // size is given in bytes, so divide by four
        let data: Vec<u8> = vec![0; size];
        return Memory {data}
    }

    pub fn write(& mut self, address: usize, value: u8) {
        self.data[address] = value;
    }

    pub fn read(& self, address:usize) -> u64 {
        return self.data[address] as u64;
    }
}