
mod rom;
mod regfile;
mod state;
mod processor;

use regfile::RegFile;
use rom::Rom;
use state::State;
use processor::Processor;

use std::env;

fn main() {
    // get command line args
    let args: Vec<String> = env::args().collect();

    // create processor components
    let mut register_file:RegFile = RegFile::new_regfile(16);
    let mut state:State = State::new_state(0, register_file);
    let mut processor:Processor = Processor::new_processor(state);

    // read ROM
    let rom = Rom::new_rom(args[1].clone());

    for address in (0..rom.get_length()) {
        println!("ROM[{:#04x}] = {:#08x}", address, rom.get_instruction(address));
    }

}  

