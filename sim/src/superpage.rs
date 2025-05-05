use std::ptr::NonNull;

#[cfg(target_os = "macos")]
mod os {
    use libc::{kern_return_t, mach_port_t, vm_address_t, vm_size_t};
    use std::ptr::NonNull;

    const VM_FLAGS_SUPERPAGE_SIZE_2MB: u32 = 0x80000000;
    const VM_FLAGS_ANYWHERE: u32 = 0x0001;

    extern "C" {
        fn mach_task_self() -> mach_port_t;

        fn vm_allocate(
            target_task: mach_port_t,
            address: *mut vm_address_t,
            size: vm_size_t,
            flags: i32,
        ) -> kern_return_t;
    }

    pub fn alloc_superpage(size: usize) -> Result<NonNull<u8>, String> {
        unsafe {
            let mut addr: vm_address_t = 0;
            let result = vm_allocate(
                mach_task_self(),
                &mut addr,
                size as vm_size_t,
                (VM_FLAGS_ANYWHERE | VM_FLAGS_SUPERPAGE_SIZE_2MB) as i32,
            );

            if result == 0 {
                NonNull::new(addr as *mut u8).ok_or("Null pointer after vm_allocate".into())
            } else {
                Err(format!("vm_allocate failed with code: {}", result))
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod os {
    use libc::{
        madvise, mmap, munmap, MAP_ANONYMOUS, MAP_FAILED, MAP_HUGETLB, MAP_PRIVATE, PROT_READ,
        PROT_WRITE, MADV_HUGEPAGE,
    };
    use std::ptr::NonNull;

    const MAP_HUGE_SHIFT: usize = 26;
    const MAP_HUGE_2MB: i32 = 21 << MAP_HUGE_SHIFT; // 2MB huge page

    pub fn alloc_superpage(size: usize) -> Result<NonNull<u8>, String> {
        unsafe {
            // First attempt: mmap with MAP_HUGETLB (explicit hugepages)
            let ptr = mmap(
                std::ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB | MAP_HUGE_2MB,
                -1,
                0,
            );

            if ptr != MAP_FAILED {
                return NonNull::new(ptr as *mut u8)
                    .ok_or_else(|| "Null pointer from mmap with MAP_HUGETLB".to_string());
            }

            // Fallback: mmap without MAP_HUGETLB + madvise(MADV_HUGEPAGE)
            let ptr = mmap(
                std::ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            );

            if ptr == MAP_FAILED {
                return Err("mmap fallback failed".into());
            }

            // Hint the kernel to back with transparent hugepages
            madvise(ptr, size, MADV_HUGEPAGE);

            NonNull::new(ptr as *mut u8).ok_or("Null pointer after mmap fallback".into())
        }
    }
}

use os::alloc_superpage;

/// A block of memory backed by large pages (if the OS allows).
pub struct Superpage {
    ptr: NonNull<u8>,
    size: usize,
}

impl Superpage {
    pub fn new(size: usize) -> Result<Self, String> {
        let ptr = alloc_superpage(size)?;
        unsafe {
            let mut p = ptr.as_ptr();
            let end = p.add(size);
            while p < end {
                std::ptr::write_volatile(p, 0);
                p = p.add(4096); // touch every 4KB page
            }
        }
        Ok(Self { ptr, size })
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size) }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn write_from_buffer(&mut self, index: usize, buffer: &[u8]) {
        unsafe {
            let target_ptr = self.ptr.as_ptr().add(index);
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), target_ptr, buffer.len());
        }
    }

    pub fn read_to_buffer(&self, index: usize, buffer: &mut [u8]) {
        unsafe {
            let target_ptr = self.ptr.as_ptr().add(index);
            std::ptr::copy_nonoverlapping(target_ptr, buffer.as_mut_ptr(), buffer.len());
        }
    }
}

unsafe impl Send for Superpage {}
unsafe impl Sync for Superpage {}

impl std::fmt::Debug for Superpage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Superpage")
            .field("ptr", &self.ptr)
            .field("size", &self.size)
            .finish()
    }
}