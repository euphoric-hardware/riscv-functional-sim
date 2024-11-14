use crate::cpu::{Exception, Result};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    fs,
};

#[derive(Clone, Copy, Default, Debug)]
struct MemoryRange {
    base_address: u64,
    size: u64,
}

impl PartialEq for MemoryRange {
    fn eq(&self, other: &Self) -> bool {
        self.base_address.eq(&other.base_address)
    }
}

impl Eq for MemoryRange {}

impl PartialOrd for MemoryRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.base_address.partial_cmp(&other.base_address)
    }
}

impl Ord for MemoryRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.base_address.cmp(&other.base_address)
    }
}

impl MemoryRange {
    fn contains(&self, addr: u64) -> bool {
        addr >= self.base_address && addr < self.base_address + self.size
    }
}

pub trait Device: Debug {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()>;
    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()>;
}

#[derive(Debug)]
pub struct Bus<'b> {
    devices: BTreeMap<MemoryRange, Box<dyn Device + 'b>>,
}

impl<'b> Bus<'b> {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
        }
    }

    fn get_device(&mut self, addr: u64) -> Result<(&MemoryRange, &mut (dyn Device + 'b))> {
        self.devices
            .range_mut(
                ..=MemoryRange {
                    base_address: addr,
                    ..Default::default()
                },
            )
            .rev()
            .find(|(range, _)| range.contains(addr)) // should be first
            .map(|(r, device)| (r, &mut **device))
            .ok_or(Exception::LoadAccessFault)
    }

    pub fn register(&mut self, device: Box<dyn Device>, base_address: u64, size: u64) {
        self.devices
            .insert(MemoryRange { base_address, size }, device);
    }
}

impl Device for Bus<'_> {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()> {
        let (memory_range, device) = self.get_device(ptr)?;
        device.read(ptr - memory_range.base_address, buf)
    }

    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()> {
        let (memory_range, device) = self.get_device(ptr)?;
        device.write(ptr - memory_range.base_address, buf)
    }
}

#[derive(Default, Debug)]
pub struct Ram {
    // Vec size: 4096
    sparse_memory_map: HashMap<u64, Vec<u8>>,
}

impl Ram {
    const PAGE_SIZE: u64 = 0x1000;
    const PAGE_OFFSET_BITS: u64 = 12; // log(PAGE_SIZE)
}

impl Ram {
    fn page_slice(&mut self, ptr: u64, len: u64) -> &mut [u8] {
        let (page_id, page_offset) = (
            ptr >> Self::PAGE_OFFSET_BITS,
            ptr & ((1 << Self::PAGE_OFFSET_BITS) - 1),
        );

        &mut self
            .sparse_memory_map
            .entry(page_id)
            .or_insert(vec![0; Self::PAGE_SIZE as usize])
            [page_offset as usize..page_offset as usize + len as usize]
    }
}

impl Device for Ram {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()> {
        buf.copy_from_slice(&self.page_slice(ptr, buf.len() as u64));
        Ok(())
    }

    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()> {
        self.page_slice(ptr, buf.len() as u64).copy_from_slice(&buf);
        Ok(())
    }
}
