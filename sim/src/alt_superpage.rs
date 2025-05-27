use libc::{
    madvise, mlock, mmap, munmap, MADV_HUGEPAGE, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_READ,
    PROT_WRITE,
};
use std::{ptr, slice};

pub fn alloc_locked_huge_memory(size: usize) -> *mut u8 {
    unsafe {
        // mmap the memory
        let ptr = mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON,
            -1,
            0,
        );

        if ptr == MAP_FAILED {
            panic!("mmap failed");
        }

        // Advise kernel to use huge pages
        if madvise(ptr, size, MADV_HUGEPAGE) != 0 {
            panic!("madvise failed");
        }

        // Lock the memory to avoid paging
        if mlock(ptr, size) != 0 {
            panic!("mlock failed");
        }

        // Prefault: write to each page to force allocation
        let page_size = 4096; // or 2 * 1024 * 1024 for 2MiB huge pages
        let slice = slice::from_raw_parts_mut(ptr as *mut u8, size);
        for i in (0..size).step_by(page_size) {
            slice[i] = 0;
        }

        return ptr as *mut u8;
    }

    // Optional cleanup
    pub unsafe fn free_locked_memory(ptr: *mut u8, size: usize) {
        munmap(ptr as *mut _, size);
    }
}

pub unsafe fn write_to_memory(dest_ptr: *mut u8, buf: &[u8]) {
    ptr::copy_nonoverlapping(buf.as_ptr(), dest_ptr, buf.len());
}

/// Reads memory starting at `src_ptr` into `buf`.
/// Safety: Caller must ensure `src_ptr` is valid and readable for `buf.len()` bytes.
pub unsafe fn read_from_memory(src_ptr: *const u8, buf: &mut [u8]) {
    ptr::copy_nonoverlapping(src_ptr, buf.as_mut_ptr(), buf.len());
}