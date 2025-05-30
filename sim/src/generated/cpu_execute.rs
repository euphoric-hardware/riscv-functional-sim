use std::ptr::null;

use crate::{
    branch_hints::unlikely,
    bus::{Bus, Device},
    cpu::{self, Cpu, Exception, Insn},
    insn_impl::{self, jump_table},
    uop_cache::{self, uop_cache::UopCacheEntry},
};

impl Cpu {
    #[inline(never)]
    pub fn execute_insn(&mut self, bus: &mut Bus) -> cpu::Result<u64> {
        let index = ((self.pc - self.uop_base) / self.uop_stride) as usize;

        if let Some(Some(entry)) = self.uop_cache.get(index) {
            let entry_ptr = entry as *const UopCacheEntry;
            self.cache_hits += 1;
            unsafe {
                jump_table::JUMP_TABLE[(*entry_ptr).jump_table_index](self, bus, &*entry_ptr)
            }
        } else {
            let mut bytes = [0; std::mem::size_of::<u32>()];
            bus.read(self.pc, &mut bytes)?;
            let insn = Insn::from_bytes(&bytes);

            let entry = UopCacheEntry::new(insn);
            if let Some(entry) = entry {
                if index >= self.uop_cache.len() {
                    self.uop_cache.resize(index + 1, None);
                }
                self.uop_cache[index] = Some(entry);
                self.execute_insn(bus)
            } else {
                Err(cpu::Exception::IllegalInstruction)
            }
        }
    }
}
