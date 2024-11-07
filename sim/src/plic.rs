use crate::bus::{self, Device};

#[derive(Debug)]
pub struct Plic {}

impl Device for Plic {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> bus::Result<()> {
        todo!()
    }

    fn write(&mut self, ptr: u64, buf: &[u8]) -> bus::Result<()> {
        todo!()
    }
}
