#![allow(warnings)]
mod rom;
mod instructions;
mod disassembler;
mod csr;
mod csr_addresses;
mod processor;

use disassembler::Disassembler;
use rom::Rom;
use csr::{CSR, Field, Access, Privilege};
use csr_addresses::*;
use processor::Processor;

use std::{cmp::max, env, mem};

fn main() {
    // get command line args
    let args: Vec<String> = env::args().collect();

    // read ROM
    let rom = Rom::new_rom(args[1].clone());
    
    // create processor
    let mut processor:Processor = Processor::new_processor(0xffff, &rom);

    // add example csr
    let mut mepc = CSR::new_csr(String::from("mepc"), Privilege::M);
    mepc.add_field(64, 0, Access::WARL, u64::max_value() - 1);  

    processor.add_csr(mepc, csr_addresses::CSR_MEPC); 

    // create disassembler
    let disassembler: Disassembler = Disassembler::new_disassembler(processor.get_instruction_memory());

    while (processor.get_pc() as usize) < rom.get_length() {
        println!("{trace}", trace = disassembler.get_trace(processor.get_pc() as usize));
        processor.step();
    }

    processor.display_state();
}

