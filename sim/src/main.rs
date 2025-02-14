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

use std::path::Path;

use generated::cpu_execute as _;
pub use log::*;

use fesvr::frontend::Frontend;

fn main() {
    env_logger::init();

    let dir = Path::new("test_bench");

    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            file_name_str.starts_with("test")
                && file_name_str.contains("")
                && file_name_str.ends_with(".elf")
        })
        .collect();

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in entries {
        println!("testing... {}", entry.file_name().to_string_lossy());
        let mut system = system::System::new();
        system.cpus[0]
            .csrs
            .store_unchecked(csrs::Csrs::MSTATUS, 0b00000000000000000001100000000000);
        let mut frontend = Frontend::try_new(dir.join(&entry.file_name())).unwrap();
        frontend.write_elf(&mut system).unwrap();

        let mut i = 1;
        loop {
            system.tick();
            if i % 50 == 0 {
                if frontend.process(&mut system).expect("htif") {
                    break;
                }
            }
            let minstret = system.cpus[0].csrs.load(csrs::Csrs::MINSTRET).expect("nonexistent csr!");
            system.cpus[0].csrs.store(csrs::Csrs::MINSTRET, minstret + 1);

            i += 1;
        }
        // println!("minstret = {}", system.cpus[0].csrs.load(csrs::Csrs::MINSTRET).expect("nonexistent csr!"));
    }
    
}
