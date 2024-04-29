use core::{alloc::{GlobalAlloc, Layout}, ffi::c_void};

extern "C" {
    pub fn malloc(arg1: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn free(arg1: *mut ::core::ffi::c_void);
}

struct DSAllocator;

unsafe impl GlobalAlloc for DSAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void);
    }
}

#[global_allocator]
static ALLOCATOR: DSAllocator = DSAllocator;