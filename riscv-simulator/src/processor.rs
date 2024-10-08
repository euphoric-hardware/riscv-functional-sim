use crate::{regfile::RegFile, state::State};
pub struct Processor {
    state: State
}

impl Processor {
    pub fn new_processor(state: State) -> Processor {
        Processor {state}
    }    

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn get_state(&self) -> &State {
        return &self.state;
    }

    // display pc and registers for basic debugging
    pub fn display_state(&self) {
        println!("Current PC = {:#04x}", self.state.get_pc());
        for i in (0..self.state.get_regfile().get_num_registers()) {
            println!("\tr{i} = {value}", value=self.state.get_regfile().read(i));
        }
    }
    
}