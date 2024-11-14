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

use generated::cpu_execute as _;
pub use log::*;

#[cfg(test)]
mod tests {
    use fesvr::frontend::Frontend;

    use super::system::System;

    use std::{fs, sync::Once};

    static INIT: Once = Once::new();
    fn setup() {
        INIT.call_once(|| {
            ::env_logger::init();
        });
    }

    #[test]
    fn run_rom() {
        setup();

        let mut system = System::new();
        let mut frontend = Frontend::try_new("rv64ui-p-add").unwrap();
        frontend.write_elf(&mut system).unwrap();

        let mut i = 1;
        loop {
            system.tick();
            if i % 50 == 0 {
                frontend.process(&mut system).expect("htif");
            }

            i += 1;
        }
    }
}
