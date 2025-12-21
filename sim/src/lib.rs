#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(cold_path)]
#![feature(likely_unlikely)]

pub mod bus;
pub mod cpu;
pub mod csrs;
pub mod diff;
mod generated;
mod insn_impl;
pub mod mmu;
pub mod plic;
pub mod superpage;
pub mod system;
mod uop_cache;

pub use branch_hints;
use bus::{Bus, Device, Ram};
use once_cell::sync::OnceCell;

pub static DIFF: OnceCell<bool> = OnceCell::new();
pub static LOG: OnceCell<bool> = OnceCell::new();

pub fn init() {
    let _ = DIFF.set(true);
    let _ = LOG.set(false);
}

pub struct RefCore<'a> {
    pub cpu: cpu::Cpu,
    pub bus: Bus<'a>,
}

impl<'a> RefCore<'a> {
    pub fn new(base_addr: u64, size: u64) -> Self {
        init();

        let mut bus = Bus::new();
        let ram = Ram::new(base_addr, size);
        bus.register(Box::new(ram), base_addr, size);

        let sign_extended_base = base_addr | 0xffffffff00000000;
        let ram_sign_ext = Ram::new(sign_extended_base, size);
        bus.register(Box::new(ram_sign_ext), sign_extended_base, size);

        let mut cpu = cpu::Cpu::new();
        cpu.pc = base_addr;

        RefCore { cpu, bus }
    }

    pub fn load_instructions(&mut self, base_addr: u64, instructions: &[u32]) {
        for (i, insn) in instructions.iter().enumerate() {
            let addr = base_addr + (i as u64) * 4;
            let bytes = insn.to_le_bytes();
            self.bus.write(addr, &bytes).expect("failed to write instruction");
        }
    }

    pub fn step(&mut self) -> StepResult {
        let old_pc = self.cpu.pc;
        self.cpu.step(&mut self.bus);

        let wb_valid = !self.cpu.commits.reg_write.is_empty();
        let (wb_rd, wb_data) = if wb_valid {
            let (&rd, &data) = self.cpu.commits.reg_write.iter().next().unwrap();
            self.cpu.commits.reg_write.clear();
            (rd, data)
        } else {
            (0, 0)
        };

        StepResult {
            pc: old_pc,
            next_pc: self.cpu.pc,
            wb_valid,
            wb_rd,
            wb_data,
        }
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.cpu.pc = pc;
    }

    pub fn get_pc(&self) -> u64 {
        self.cpu.pc
    }

    pub fn get_reg(&self, idx: u64) -> u64 {
        self.cpu.load(idx)
    }

    pub fn set_reg(&mut self, idx: u64, val: u64) {
        self.cpu.regs[idx as usize] = val;
    }

    pub fn dump_state(&self) {
        println!("PC:  0x{:016x}", self.cpu.pc);
        println!("Registers:");
        for i in 0..32 {
            let val = self.cpu.regs[i];
            if i % 4 == 0 {
                print!("  ");
            }
            print!("x{:<2}: 0x{:016x}  ", i, val);
            if i % 4 == 3 {
                println!();
            }
        }
    }

    pub fn get_regs(&self) -> [u64; 32] {
        self.cpu.regs
    }
}

#[derive(Debug, Clone)]
pub struct StepResult {
    pub pc: u64,
    pub next_pc: u64,
    pub wb_valid: bool,
    pub wb_rd: u64,
    pub wb_data: u64,
}
