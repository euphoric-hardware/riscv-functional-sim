use log::info;

use crate::cpu::{Exception, Result};
use lazy_static::lazy_static;
use std::{
    cmp,
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
    fn contains(&self, addr: u64, len: u64) -> bool {
        addr >= self.base_address && addr.wrapping_add(len) < self.base_address + self.size
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

    fn get_device(
        &mut self,
        addr: u64,
        len: u64,
    ) -> Result<(&MemoryRange, &mut (dyn Device + 'b))> {
        self.devices
            .range_mut(
                ..=MemoryRange {
                    base_address: addr,
                    ..Default::default()
                },
            )
            .rev()
            .find(|(range, _)| range.contains(addr, len)) // should be first
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
        match self.get_device(ptr, buf.len() as u64) {
            Ok((memory_range, device)) => {
                // Proceed normally
                device.read(ptr - memory_range.base_address, buf)
            }
            Err(e) => {
                println!("get_device failed: {:?} at address {:#016x}", e, ptr);
                return Err(e); // Or handle it in another way
            }
        }
        // device.read(ptr - memory_range.base_address, buf)
    }

    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()> {
        let (memory_range, device) = self
            .get_device(ptr, buf.len() as u64)
            .expect(&format!("device does not exist, ptr: 0x{:X}", ptr));
        device.write(ptr - memory_range.base_address, buf)
    }
}

#[derive(Default, Debug)]
pub struct Ram {
    // Vec size: 4096
    sparse_memory_map: HashMap<u64, Vec<u8>>,
}

impl Ram {
    pub const PAGE_SIZE: u64 = 0x1000;
    pub const PAGE_OFFSET_BITS: u64 = 12; // log(PAGE_SIZE)

    
}

lazy_static! {
    static ref ZERO_PAGE: Vec<u8> = vec![0u8; Ram::PAGE_SIZE as usize];
}
impl Ram {
    fn page_slice(&mut self, ptr: u64, len: u64) -> &mut [u8] {
        let (page_id, page_offset) = (
            ptr >> Self::PAGE_OFFSET_BITS,
            ptr & ((1 << Self::PAGE_OFFSET_BITS) - 1),
        );
    
        let page = self.sparse_memory_map.entry(page_id)
            .or_insert_with(|| ZERO_PAGE.clone());
    
        &mut page[page_offset as usize..page_offset as usize + len as usize]
    }
}

impl Device for Ram {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()> {
        let mut ptr = ptr;
        let mut remaining = buf;

        while !remaining.is_empty() {
            let page_offset = ptr & ((1 << Self::PAGE_OFFSET_BITS) - 1);

            let bytes_in_page = Self::PAGE_SIZE as usize - page_offset as usize;
            let to_read = cmp::min(remaining.len(), bytes_in_page);
            let (chunk, rest) = remaining.split_at_mut(to_read);

            chunk.copy_from_slice(self.page_slice(ptr, to_read as u64));

            ptr += to_read as u64;
            remaining = rest;
        }

        Ok(())
    }

    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()> {
        let mut ptr = ptr;
        let mut remaining = buf;

        while !remaining.is_empty() {
            let page_offset = ptr & ((1 << Self::PAGE_OFFSET_BITS) - 1);

            let bytes_in_page = Self::PAGE_SIZE as usize - page_offset as usize;
            let to_write = cmp::min(remaining.len(), bytes_in_page);
            let (chunk, rest) = remaining.split_at(to_write);

            self.page_slice(ptr, to_write as u64).copy_from_slice(chunk);

            ptr += to_write as u64;
            remaining = rest;
        }

        Ok(())
    }
}
