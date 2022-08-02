use core::{alloc::GlobalAlloc, sync::atomic::{AtomicBool, Ordering}};

use alloc::format;

pub struct Alloc;

#[global_allocator]
pub static mut ALLOC: Alloc = Alloc;

static NESTED: AtomicBool = AtomicBool::new(false);

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let nested = NESTED.swap(true, Ordering::Relaxed);
        let res = alloc(layout.size(), layout.align(), true);
        if !nested {
            crate::uart0::send(format!("Allocated at 0x{:X}", res as usize).as_bytes());
            NESTED.store(false, Ordering::Relaxed);
        }
        res
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        dealloc(ptr);
    }
}

extern "C" {
    pub fn heap_init();
    fn alloc(size: usize, align: usize, try_compact: bool) -> *mut u8;
    fn dealloc(ptr: *mut u8);
}

#[alloc_error_handler]
fn __alloc_error(_: core::alloc::Layout) -> ! {
    crate::uart0::send(b"Out of memory!");
    panic!();
}
