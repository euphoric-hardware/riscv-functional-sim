use crate::regfile::RegFile;
use serde::{Deserialize, Serialize};

#[derive(serde::Serialize)]
pub struct State<'a> {
    pc: u64,                  // program counter
    register_file: &'a mut RegFile, // register file
}

impl<'a> State<'a> {
    // constructor
    pub fn new_state(pc: u64, register_file: &'a mut RegFile) -> Self {
        State {
            pc: pc,
            register_file: register_file,
        }
    }

    pub fn get_pc(&self) -> u64 {
        return self.pc;
    }
    
    pub fn set_pc(&mut self, value: u64) {
        self.pc = value;
    }

    pub fn get_regfile(&mut self) -> &mut RegFile {
        return self.register_file;
    }

    pub fn increment_pc(& mut self) {
        self.pc += 4;
    }
    
    pub fn display_state(&mut self) {
        // display pc and registers for basic debugging
        println!("Current PC = {:#04x}", self.get_pc());
        for i in 0..self.get_regfile().get_num_registers() {
            println!(
                "r{i} = {value}",
                value = self.get_regfile().read(i) as i64
            );
        }
    }

}
