use ahash::AHashMap;
use log::info;

use crate::cpu::{Exception, Result};
use lazy_static::lazy_static;
use std::{
    cmp::{self, Ordering},
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
    devices: Vec<(MemoryRange, Box<dyn Device + 'b>)>,
}

impl<'b> Bus<'b> {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    fn get_device(
        &mut self,
        addr: u64,
        len: u64,
    ) -> Result<(&MemoryRange, &mut (dyn Device + 'b))> {
        match self.devices.binary_search_by(|(range, _)| {
            if addr < range.base_address {
                Ordering::Greater
            } else if addr >= range.base_address + range.size {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(index) => {
                let (range, device) = &mut self.devices[index];
                Ok((range, &mut **device))
            }
            Err(_) => Err(Exception::LoadAccessFault),
        }
    }

    pub fn register(&mut self, device: Box<dyn Device>, base_address: u64, size: u64) {
        self.devices
            .push((MemoryRange { base_address, size }, device));
        self.devices.sort_by_key(|(range, _)| range.base_address);
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

type Page = Box<[u8; Ram::PAGE_SIZE as usize]>;
#[derive(Default, Debug)]
pub struct Ram {
    // Vec size: 4096
    sparse_memory_map: AHashMap<u64, Page>,
}

impl Ram {
    pub const PAGE_SIZE: u64 = 0x1000;
    pub const PAGE_OFFSET_BITS: u64 = 12; // log(PAGE_SIZE)
    pub const PAGE_MASK: u64 = (1 << Self::PAGE_OFFSET_BITS) - 1;
}

impl Ram {
    fn create_empty_page() -> Page {
        vec![0u8; Ram::PAGE_SIZE as usize]
            .into_boxed_slice()
            .try_into()
            .unwrap_or_else(|_| panic!("Incorrect page size"))
    }

    fn page_slice(&mut self, ptr: u64, len: u64) -> &mut [u8] {
        let (page_id, page_offset) = (
            ptr >> Self::PAGE_OFFSET_BITS,
            ptr & ((1 << Self::PAGE_OFFSET_BITS) - 1),
        );

        let page = self
            .sparse_memory_map
            .entry(page_id)
            .or_insert_with(Self::create_empty_page);

        let start = page_offset as usize;
        let end = start + len as usize;

        if end > Ram::PAGE_SIZE as usize {
            panic!(
                "page_slice out of bounds: offset {} + len {} > page size {}",
                page_offset,
                len,
                Ram::PAGE_SIZE
            );
        }
        &mut page[start..end]
    }
}

impl Device for Ram {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()> {
        let mut offset = 0;
        let mut ptr = ptr;
        while offset < buf.len() {
            let page_offset = (ptr & Self::PAGE_MASK) as usize;
            let bytes_in_page = Self::PAGE_SIZE - page_offset as u64;
            let to_read = cmp::min(buf.len() - offset, bytes_in_page as usize);

            let dst = &mut buf[offset..offset + to_read];
            let src = &self.page_slice(ptr, to_read as u64)[..to_read];

            dst.copy_from_slice(src);

            ptr += to_read as u64;
            offset += to_read;
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
