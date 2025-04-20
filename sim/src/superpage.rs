use libc::{kern_return_t, mach_port_t, vm_address_t, vm_size_t};
use std::ptr::NonNull;

/// VM allocation flags for 2MB superpages on macOS
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

/// Allocates memory with a superpage request (2MB-aligned, huge pages if possible).
fn alloc_superpage(size: usize) -> Result<NonNull<u8>, String> {
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

/// A block of memory backed by large pages (if the OS allows).
pub struct Superpage {
    ptr: NonNull<u8>,
    size: usize,
}

impl Superpage {
    /// Attempts to allocate `size` bytes of memory, aligned to 2MB pages.
    pub fn new(size: usize) -> Result<Self, String> {
        let ptr = alloc_superpage(size)?;
        unsafe {
            ptr.as_ptr().write_bytes(0, size); // Touch pages, zero init
        }
        Ok(Self { ptr, size })
    }

    /// Returns a raw mutable pointer to the start of the memory.
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Returns a mutable slice to the allocated memory.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size) }
    }

    /// Returns the size of the memory block.
    pub fn len(&self) -> usize {
        self.size
    }

    // Write data from a buffer to a specific index in the allocated memory
    pub fn write_from_buffer(&mut self, index: usize, buffer: &[u8]) {
        unsafe {
            let ptr = self.as_mut_ptr();
            let target_ptr = ptr.add(index);

            // Copy data from the buffer into the superpage memory
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), target_ptr, buffer.len());
        }
    }

    // Read data from the allocated memory into a buffer
    pub fn read_to_buffer(&self, index: usize, buffer: &mut [u8]) {
        unsafe {
            let ptr = self.as_mut_ptr();
            let target_ptr = ptr.add(index);

            // Copy data from the superpage memory into the buffer
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
