use crate::bus::{Bus, Ram};
use crate::cpu::Cpu;

pub struct System<'b> {
    cpus: Vec<Cpu>,
    bus: Bus<'b>,
}

impl System<'_> {
    // testver
    pub fn new() -> Self {
        let mut bus = Bus::new();
        let ram = Ram::default();

        bus.register(Box::new(ram), 0x8000000, 0x1000);
        Self {
            bus,
            cpus: vec![Cpu::new()],
        }
    }

    pub fn from_dtb() -> Self {
        todo!()
    }

    pub fn tick(&mut self) {
        self.cpus[0].step(&mut self.bus);
    }
}
