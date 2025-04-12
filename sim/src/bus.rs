use ahash::AHashMap;
use clap::builder::StringValueParser;
use log::info;

use crate::cpu::{Exception, Result};
use lazy_static::lazy_static;
use std::{
    cmp::{self, Ordering},
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    fs, mem,
    thread::panicking,
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
            .expect("device does not exist");
        device.write(ptr - memory_range.base_address, buf)
    }
}

type Page = Box<[u8; Ram::PAGE_SIZE as usize]>;
#[derive(Debug)]
pub struct Ram {
    // Vec size: 4096
    base_address: u64,
    size: u64,
    memory: Vec<u8>,
}

impl Ram {
    pub const PAGE_SIZE: u64 = 0x1000;
    pub const PAGE_OFFSET_BITS: u64 = 12; // log(PAGE_SIZE)
    pub const PAGE_MASK: u64 = (1 << Self::PAGE_OFFSET_BITS) - 1;
}

impl Ram {
    pub fn new(base_address: u64, size: u64) -> Self {
        Ram {
            base_address: base_address,
            size: size,
            memory: vec![0u8; size as usize],
        }
    }
    fn create_empty_page() -> Page {
        vec![0u8; Ram::PAGE_SIZE as usize]
            .into_boxed_slice()
            .try_into()
            .unwrap_or_else(|_| panic!("Incorrect page size"))
    }

    // fn page_slice(&mut self, ptr: u64, len: u64) -> &mut [u8] {
    //     let (page_id, page_offset) = (
    //         ptr >> Self::PAGE_OFFSET_BITS,
    //         ptr & ((1 << Self::PAGE_OFFSET_BITS) - 1),
    //     );

    //     let page = self
    //         .sparse_memory_map
    //         .entry(page_id).or_insert_with(Self::create_empty_page);

    //     let start = page_offset as usize;
    //     let end = start + len as usize;

    //     if end > Ram::PAGE_SIZE as usize {
    //         panic!("page_slice: out of bounds access");
    //     }
    //     &mut page[start..end]
    // }
}

impl Device for Ram {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()> {
        let addr = (ptr % self.base_address);
        let end = addr
            .checked_add(buf.len() as u64)
            .expect("Address overflow");
        if end > self.memory.len() as u64 {
            panic!("out of bounds read!");
        }

        buf.copy_from_slice(&&self.memory[addr as usize..end as usize]);
        Ok(())
    }

    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()> {
        let addr = (ptr % self.base_address);
        let end = addr
            .checked_add(buf.len() as u64)
            .expect("Address overflow");
        if end > self.memory.len()as u64 {
            panic!("Write out of bounds");
        }

        self.memory[addr as usize..end as usize].copy_from_slice(buf);
        Ok(())
    }
}
