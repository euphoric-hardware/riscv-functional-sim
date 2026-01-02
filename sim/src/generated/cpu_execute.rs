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
            self.uop_stride = 4;
        } else if self.pc < self.uop_base {
            self.uop_cache.clear();
            self.uop_base = self.pc;
            self.uop_stride = 4;
        }

        let index = ((self.pc - self.uop_base) >> 1) as usize;

        match self.uop_cache.get(index) {
            Some(entry) if entry.valid => {
                let entry_ptr = entry as *const UopCacheEntry;
                crate::bus::clear_last_data_access();
                crate::bus::set_record_data_access(true);
                let result = unsafe { (*entry_ptr).execute_cached_insn(self, bus) };
                crate::bus::set_record_data_access(false);
                result
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
