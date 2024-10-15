mod cpu;
mod generated;
mod insn_impl;
mod log;

use generated::cpu_execute as _;
pub use log::*;

#[cfg(test)]
mod tests {
    use super::cpu::{Cpu, Insn};

    use std::sync::Once;

    static INIT: Once = Once::new();
    fn setup() {
        INIT.call_once(|| {
            ::env_logger::init();
        });
    }

    #[test]
    fn it_works() {
        setup();

        let mut cpu = Cpu::default();
        cpu.regs[1] = 123;
        cpu.regs[2] = 456;
        cpu.execute(Insn(0x002082b3)); // add x5, x1, x2

        println!("{:?}", cpu.regs);
        assert_eq!(cpu.regs[5], 123 + 456)
    }
}
