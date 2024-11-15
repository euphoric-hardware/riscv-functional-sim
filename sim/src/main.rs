#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod bus;
mod cpu;
mod csrs;
mod generated;
mod insn_impl;
mod log;
mod plic;
mod system;

use std::path::Path;

use generated::cpu_execute as _;
pub use log::*;

use fesvr::frontend::Frontend;

fn main() {
    env_logger::init();

    let dir = Path::new("../../riscv-tests/isa/");

    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            file_name_str.starts_with("rv64ui-p-") && !file_name_str.ends_with(".dump")
        })
        .collect();

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in entries {
        println!("testing... {}", entry.file_name().to_string_lossy());
        let mut system = system::System::new();
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

            i += 1;
        }
    }

    println!("rv64ui-p-* tests passed!");
}
