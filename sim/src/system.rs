use fesvr::Htif;

use crate::bus::{Bus, Device, Ram};
use crate::Frontend;
use crate::cpu::Cpu;

pub struct System<'b> {
    pub cpus: Vec<Cpu>,
    pub bus: Bus<'b>,
}

impl<'b> System<'b> {
    // testver
    pub fn new() -> Self {
        let mut bus = Bus::new();
        let ram = Ram::new(0x80000000,(1 << 29));

        bus.register(Box::new(ram), 0x80000000, (1 << 29));

        let mut cpu = Cpu::new();
        cpu.pc = 0x80000000;
        
        Self {
            bus,
            cpus: vec![cpu],
        }
    }

    pub fn from_dtb() -> Self {
        todo!()
    }

    pub fn tick(&mut self) {
        self.cpus[0].step(&mut self.bus);
    }
}

impl Htif for System<'_> {
    fn align(&self) -> u64 {
        4
    }

    fn max_chunk_bytes(&self) -> u64 {
        1024
    }

    fn read_chunk(&mut self, ptr: u64, buf: &mut [u8]) -> fesvr::Result<()> {
        self.bus.read(ptr, buf).map_err(|_| fesvr::Error::Misc)
    }

    fn write_chunk(&mut self, ptr: u64, buf: &[u8]) -> fesvr::Result<()> {
        self.bus.write(ptr, buf).map_err(|_| fesvr::Error::Misc)
    }
}
