use crate::regfile::RegFile;

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

}
