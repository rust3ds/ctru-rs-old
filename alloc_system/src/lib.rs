#![feature(allocator)]
#![feature(no_std)]
#![no_std]
#![allocator]
#![allow(improper_ctypes)]
#![allow(unused_variables)]

extern "C" {
    pub fn memalign(align: usize, size: usize) -> *mut u8;
    pub fn malloc(size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);
    pub fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe { malloc(size) }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe { free(ptr) }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                                align: usize) -> *mut u8 {
    unsafe { realloc(ptr, size) }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(ptr: *mut u8, old_size: usize,
                                        size: usize, align: usize) -> usize {
    old_size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, align: usize) -> usize {
    size
}
