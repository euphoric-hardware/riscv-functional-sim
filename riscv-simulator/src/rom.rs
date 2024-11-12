use std::fs::File;
use std::io::prelude::*;

pub struct Rom {
    data: Vec<u8>,
    length: usize,
}

impl Rom {
    pub fn new_rom(filename: String) -> Self {
        let mut data: Vec<u8> = (0..0).collect();

        let f = File::open(filename).expect("Unable to open ROM\n");
        let mut i: usize = 0;

        for byte_or_error in f.bytes() {
            let byte = byte_or_error.unwrap();
            data.push(byte);
            i += 1;
        }

        let length = data.len();

        Rom {
            data: data,
            length: length,
        }
    }

    pub fn get_length(&self) -> usize {
        return self.length;
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        return self.data[address as usize] as u8;
    }

    pub fn read_word(&self, address: usize) -> u32 {
        return (self.data[address + 3 as usize] as u32 | (self.data[address + 2 as usize] as u32 ) << 8 | (self.data[address + 1 as usize] as u32) << 16 | (self.data[address as usize] as u32) << 24) as u32;
    }
}
