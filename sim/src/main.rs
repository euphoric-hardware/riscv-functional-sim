#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod args;
mod bus;
mod cpu;
mod csrs;
mod diff;
mod generated;
mod insn_impl;
mod logger;
mod mmu;
mod plic;
mod system;
mod uop_cache;

use args::FunctionalSimArgs;
use clap::Parser;
use ahash;
use std::env;
use std::fs::File;
use std::path::Path;

use diff::{Diff, ExecutionState};
use fesvr::frontend::{Frontend, FrontendReturnCode};
use generated::cpu_execute as _;

fn main() -> std::io::Result<()> {
    let args = FunctionalSimArgs::parse();
    File::create(&args.output_log)?;

    logger::init_logger(true, &args.output_log.to_str().unwrap());

    let mut compare_logs = false;
    let differ: Diff;
    let mut spike_states: Vec<ExecutionState> = Vec::new();

    if let Some(spike_log_path) = &args.spike_log {
        if let Some(file_name) = spike_log_path.file_name().and_then(|f| f.to_str()) {
            compare_logs = true;
            differ = diff::Diff {};
            spike_states = differ.parse_spike_log(file_name).unwrap();
            spike_states.drain(0..5);
        }
    }

    let binary = &args.bin;
    println!("Testing... {:?}\n", binary.file_name().unwrap());

    let mut system = system::System::new();
    system.cpus[0]
        .csrs
        .store_unchecked(csrs::Csrs::MSTATUS, 0b00000000000000000001100000000000);

    // spike has an extra 5 instruction startup routine, just add 5 cycles/instructions to make up for this
    system.cpus[0].csrs.store_unchecked(csrs::Csrs::MCYCLE, 5);
    system.cpus[0].csrs.store_unchecked(csrs::Csrs::MINSTRET, 5);

    let mut frontend = Frontend::try_new(binary).unwrap();
    
    frontend.write_elf(&mut system).unwrap();
    system.cpus[0].pc = frontend.reset_vector();
    let start_pc = frontend.start_of_text();
    let end_pc = frontend.end_of_text();

    // println!("start pc: {:#16x}", start_pc);
    // println!("end_pc: {:#16x}", end_pc);
    system.cpus[0].load_uop_cache(&mut system.bus, start_pc, end_pc);
    // println!("{} entries in uop cache", system.cpus[0].uop_cache.len());
    let mut i = 1;
    loop {
        system.tick();
        if compare_logs {
            if !Diff::diff_execution_state(
                spike_states.get(i - 1),
                system.cpus[0].states.get(i - 1),
            ) && i <= spike_states.len()
            {
                println!("mismatch, exeuction ended!");
                break;
            }
        }

        if i % 5000 == 0 {
            if frontend.process(&mut system).expect("htif") == FrontendReturnCode::Exit {
                println!("Target program finished");
                break;
            }
        }

        let minstret = system.cpus[0]
            .csrs
            .load(csrs::Csrs::MINSTRET)
            .expect("nonexistent csr!");
        system.cpus[0]
            .csrs
            .store(csrs::Csrs::MINSTRET, minstret + 1);

        i += 1;
    }

    // // diff logs
    // if compare_logs {
    //     Diff::diff_execution_states(&spike_states, &system.cpus[0].states);
    //     println!("Diff complete!");
    // }

    Ok(())
}
