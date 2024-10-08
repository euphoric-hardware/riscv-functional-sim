use crate::regfile::RegFile;

pub struct State {
    pc: u64,                  // program counter
    register_file: RegFile, // register file
}

impl State {
    // constructor
    pub fn new_state(pc: u64, register_file: RegFile) -> Self {
        State {
            pc: pc,
            register_file: register_file,
        }
    }

    pub fn get_pc(&self) -> u64 {
        return self.pc;
    }

    pub fn get_regfile(&self) -> &RegFile {
        return &self.register_file;
    }

}
