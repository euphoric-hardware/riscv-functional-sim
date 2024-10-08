use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct Rom {
    data: Vec<u32>,
    length: usize
}

impl Rom {
    pub fn new_rom(filename: String) -> Self{
        let mut data: Vec<u32> = (0..0).collect();

        let f = File::open(filename).expect("Unable to open ROM\n");
        let mut i: usize = 0;
        let mut instruction: u32 = 0;
        
        for byte_or_error in f.bytes() {
            let byte = byte_or_error.unwrap();
            instruction |= u32::from(byte) << (24 - ((i & 0x3) << 3)); // append each byte to the correct place in the 32 bit instruction
            
            if i & 0x3 == 0x3 {
                data.push(instruction);
                instruction = 0;
            }
            i += 1; 
        }

        let length = data.len();

        Rom {
            data: data,
            length: length
        }
    }

    pub fn get_length(&self) -> usize{
        return self.length;
    }

    pub fn get_instruction(&self, address: usize) -> u32 {
        return self.data[address];
    }

}