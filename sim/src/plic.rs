use crate::bus::Device;
use crate::cpu::Result;

#[derive(Debug)]
pub struct Plic {}

impl Device for Plic {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()> {
        todo!()
    }

    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()> {
        todo!()
    }
}
