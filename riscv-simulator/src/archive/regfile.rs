
use serde::{Deserialize, Serialize};

#[derive(serde::Serialize)]
pub struct RegFile {
    xlen: u8,
    data: Vec<u64>
}

impl RegFile {
    pub fn new_regfile(xlen:usize) -> Self{
        RegFile {
            xlen: xlen as u8,
            data: vec![0; xlen],
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
        return self.xlen as usize;
    }
    
}