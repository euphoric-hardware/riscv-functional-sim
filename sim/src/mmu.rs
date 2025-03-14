use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn, MemData, Result},
};

pub struct Mmu {}

impl Mmu {
    pub fn read(&mut self, cpu: &mut Cpu, bus: &mut Bus, ptr: u64, buf: &mut [u8]) -> Result<()> {
        bus.read(ptr, buf)?;
        cpu.commits
            .mem_read
            .insert(ptr, MemData::from_le_bytes(buf));
        Ok(())
    }

    pub fn write(&mut self, cpu: &mut Cpu, bus: &mut Bus, ptr: u64, buf: &[u8]) -> Result<()> {
        bus.write(ptr, buf)?;
        cpu.commits
            .mem_write
            .insert(ptr, MemData::from_le_bytes(buf));
        Ok(())
    }
}
