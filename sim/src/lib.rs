#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod bus;
mod cpu;
mod csr; // alternate csr implementation
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
    use super::cpu::{Cpu, Insn};

    use std::{fs, sync::Once};

    static INIT: Once = Once::new();
    fn setup() {
        INIT.call_once(|| {
            ::env_logger::init();
        });
    }

    // TODO: add mock buses

    // #[test]
    // fn it_works() {
    //     setup();

    //     let mut cpu = Cpu::default();
    //     cpu.regs[1] = 123;
    //     cpu.regs[2] = 456;
    //     cpu.execute_insn(Insn(0x002082b3)); // add x5, x1, x2

    //     assert_eq!(cpu.regs[5], 123 + 456)
    // }

    // #[test]
    // fn run_rom() {
    //     setup();

    //     let mut cpu = Cpu::default();
    //     let rom = fs::read("test_rom").expect("test_rom not found");

    //     while cpu.pc < rom.len() as u64 {
    //         cpu.step(bus);
    //         // just handle standard 32-bit wide insns right now
    //         let insn = Insn::from_bytes(&rom[cpu.pc as usize..cpu.pc as usize + 4]);
    //         cpu.execute(insn);
    //     }
    // }
}
