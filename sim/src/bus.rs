use ahash::AHashMap;
use log::info;

use crate::{
    cpu::{Exception, Result},
    superpage::{self, Superpage},
};
use lazy_static::lazy_static;
use std::{
    cmp::{self, Ordering},
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    fs,
    ptr::NonNull,
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
        let result = self.get_device(ptr, buf.len() as u64);
        if core::hint::likely(result.is_ok()) {
            let (memory_range, device) = result.unwrap();
            device.read(ptr - memory_range.base_address, buf)
        } else {
            Err(result.unwrap_err())
        }
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
    ptr: Superpage,
}

impl Ram {
    pub const PAGE_SIZE: u64 = 0x1000;
    pub const PAGE_OFFSET_BITS: u64 = 12; // log(PAGE_SIZE)
    pub const PAGE_MASK: u64 = (1 << Self::PAGE_OFFSET_BITS) - 1;
}

impl Ram {
    pub fn new(base_address: u64, size: u64) -> Self {
        Ram {
            base_address,
            size,
            ptr: superpage::Superpage::new(size as usize).expect("unable to allocate superpage"),
        }
    }
}

impl Device for Ram {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()> {
        self.ptr.read_to_buffer(ptr as usize, buf);
        Ok(())
    }

    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()> {
        self.ptr.write_from_buffer(ptr as usize, buf);
        Ok(())
    }
}
