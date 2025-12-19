use core::hint;
use std::ptr::null;

use crate::{
    branch_hints::unlikely,
    bus::{Bus, Device},
    cpu::{self, Cpu, Exception, Insn},
    insn_impl::{self},
    uop_cache::{self, uop_cache::UopCacheEntry},
};

impl Cpu {
    #[inline(never)]
    pub fn execute_insn(&mut self, bus: &mut Bus) -> cpu::Result<u64> {
        if self.uop_cache.is_empty() {
            self.uop_base = self.pc;
            self.uop_stride = 2;
        } else {
            self.uop_cache.clear();
            self.uop_base = self.pc;
            self.uop_stride = 2;
        }
        let index = ((self.pc - self.uop_base) >> 1) as usize;

        match self.uop_cache.get(index) {
            Some(entry) => {
                let entry_ptr = entry as *const UopCacheEntry;
                unsafe { (*entry_ptr).execute_cached_insn(self, bus) }
            }

            _ => {
                core::hint::cold_path();
                let mut bytes = [0; std::mem::size_of::<u32>()];
                bus.read(self.pc, &mut bytes)?;
                let insn = Insn::from_bytes(&bytes);

                let entry = UopCacheEntry::new(insn);
                
                if core::hint::likely(entry.valid) {
                    if index >= self.uop_cache.len() {
                        let new_len = index + 1;
                        let additional = new_len - self.uop_cache.len();

                        if core::hint::unlikely(self.uop_cache.try_reserve(additional).is_err()) {
                            return entry.execute_cached_insn(self, bus);
                        }

                        // Safe to unwrap here because we just reserved space
                        self.uop_cache.extend(
                            std::iter::repeat_with(UopCacheEntry::default).take(additional),
                        );
                    }

                    self.uop_cache[index] = entry;
                    self.execute_insn(bus)
                } else {
                    Err(cpu::Exception::IllegalInstruction)
                }
            }
        }
    }
}
