mod rom;
mod instruction_memory;
mod instructions;
mod disassembler;
mod regfile;
mod state;
mod memory;
mod processor;

use disassembler::Disassembler;
use instruction_memory::InstructionMemory;
use regfile::RegFile;
use rom::Rom;
use state::State;
use memory::Memory;
use processor::Processor;

use std::{cmp::max, env, mem};

fn main() {
    // get command line args
    let args: Vec<String> = env::args().collect();

    // read ROM
    let rom = Rom::new_rom(args[1].clone());
    
    // create processor components
    let instruction_memory:InstructionMemory = InstructionMemory::new_instruction_memory(&rom);
    let mut register_file:RegFile = RegFile::new_regfile(32);
    let mut state:State = State::new_state(0, &mut register_file);
    let mut memory:Memory = Memory::new_memory(0xffffffff);
    let mut processor:Processor = Processor::new_processor(&mut state, &instruction_memory, &mut memory);

    // create disassembler
    let disassembler: Disassembler = Disassembler::new_disassembler(&instruction_memory);
    
    while (processor.get_state().get_pc() as usize) < rom.get_length() {
        println!("{} {trace}", processor.get_state().get_pc(), trace = disassembler.get_trace(processor.get_state().get_pc() as usize));
        processor.step();
    }

    processor.display_state();
}  

