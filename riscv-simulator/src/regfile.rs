use core::num;

pub struct RegFile {
    data: Vec<u64>
}

impl RegFile {
    pub fn new_regfile(num_registers:usize) -> Self{
        RegFile {
            data: vec![0; num_registers],
        }
    }

    pub fn write(&mut self, register: usize, value: u64) {
        if register != 0 {
            self.data[register] = value;
        }
    }

    pub fn read(&self, register: usize) -> u64 {
        return self.data[register];
    }

    pub fn get_num_registers(&self) -> usize {
        return self.data.len()
    }
    
}