#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod bus;
mod cpu;
mod csrs;
mod generated;
mod insn_impl;
mod log;
mod mmu;
mod plic;
mod system;
mod diff;
mod args;

use std::fs::File;
use std::path::Path;
use std::env;
use args::FunctionalSimArgs;
use clap::Parser;

use diff::Diff;
use generated::cpu_execute as _;
pub use log::*;

use fesvr::frontend::Frontend;

fn main() -> std::io::Result<()> {
    let args = FunctionalSimArgs::parse();

    let differ = diff::Diff {};

    File::create(&args.log)?;
    let spike_states = differ.parse_spike_log(args.log.to_str().unwrap()).unwrap();
    env_logger::init();

    let binary = &args.bin;
    println!("Testing... {:?}", binary.file_name().unwrap());

    let mut system = system::System::new();
    system.cpus[0]
        .csrs
        .store_unchecked(csrs::Csrs::MSTATUS, 0b00000000000000000001100000000000);

    let mut frontend = Frontend::try_new(binary).unwrap();
    frontend.write_elf(&mut system).unwrap();

    let mut i = 1;
    loop {
        system.tick();
        if i % 5000 == 0 {
            if frontend.process(&mut system).expect("htif") {
                break;
            }
        }
        let minstret = system.cpus[0].csrs.load(csrs::Csrs::MINSTRET).expect("nonexistent csr!");
        system.cpus[0].csrs.store(csrs::Csrs::MINSTRET, minstret + 1);

        i += 1;
    }

    // diff logs
    Diff::diff_execution_states(&spike_states, &system.cpus[0].states);
    println!("Diff complete!");

    Ok(())
}
