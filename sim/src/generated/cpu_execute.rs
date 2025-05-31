use std::ptr::null;

use crate::{
    branch_hints::unlikely,
    bus::{Bus, Device},
    cpu::{self, Cpu, Exception, Insn},
    insn_impl::{
        self,
        jump_table::{self, JUMP_TABLE},
    },
    uop_cache::{self, uop_cache::UopCacheEntry},
};

impl Cpu {
    #[inline(never)]
    pub fn execute_insn(&mut self, bus: &mut Bus) -> cpu::Result<u64> {
        let index = ((self.pc - self.uop_base) >> 1) as usize;

        if let Some(entry) = self.uop_cache.get(index).filter(|e| e.valid) {
            let entry_ptr = entry as *const UopCacheEntry;
            self.cache_hits += 1;
            unsafe { jump_table::JUMP_TABLE[(*entry_ptr).jump_table_index](self, bus, &*entry_ptr) }
        } else {
            let mut bytes = [0; std::mem::size_of::<u32>()];
            bus.read(self.pc, &mut bytes)?;
            let insn = Insn::from_bytes(&bytes);

            let entry = UopCacheEntry::new(insn);
            if entry.valid {
                if index >= self.uop_cache.len() {
                    let new_len = index + 1;
                    let additional = new_len - self.uop_cache.len();

                    if self.uop_cache.try_reserve(additional).is_err() {
                        return jump_table::JUMP_TABLE[entry.jump_table_index](self, bus, &entry);
                    }

                    // Safe to unwrap here because we just reserved space
                    self.uop_cache
                        .extend(std::iter::repeat_with(UopCacheEntry::default).take(additional));
                }

                self.uop_cache[index] = entry;
                self.execute_insn(bus)
            } else {
                Err(cpu::Exception::IllegalInstruction)
            }
        }
    }
}
