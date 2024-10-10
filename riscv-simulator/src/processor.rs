use crate::{state::State, instruction_memory::InstructionMemory, instructions::Immediate};
pub struct Processor<'a> {
    state: &'a mut State<'a>,
    instruction_memory: &'a InstructionMemory
}

impl<'a> Processor<'a> {
    pub fn new_processor(state: &'a mut State<'a>, instruction_memory: &'a InstructionMemory) -> Processor<'a> {
        Processor {state, instruction_memory}
    }    

    pub fn set_state(&mut self, new_state: &'a mut State<'a>) {
        self.state = new_state;
    }

    pub fn get_state(&mut self) -> &mut State<'a>{
        return self.state;
    }

    pub fn display_state(&mut self) {
        // display pc and registers for basic debugging
        println!("Current PC = {:#04x}", self.state.get_pc());
        for i in (0..self.state.get_regfile().get_num_registers()) {
            println!("\tr{i} = {value}", value=self.state.get_regfile().read(i));
        }
    }

    pub fn step(&mut self) {
        // TODO - read instruction, identify its type, execute the instruction, and increment the PC
        let instruction: u32 = self.instruction_memory.read(self.get_state().get_pc() as usize);
        let opcode: u8 = (instruction & 0x7f) as u8;
        match opcode {
            0x13 => {
                // TODO - implement immediate arithmetic instructions
                
            }
            _ => println!("ILLEGAL INSTRUCTION")
        }

    }

    
    
}