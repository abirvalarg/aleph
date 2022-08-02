#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::ptr::addr_of;

use alloc::format;
use bitvec::prelude::BitVec;
use mem::MemorySubsystem;

mod allocator;
mod watchdog;
mod uart0;
mod sync;
mod mem;

#[no_mangle]
unsafe extern "C" fn rust_start() -> ! {
    watchdog::disable();
    allocator::heap_init();
    uart0::send(b"Starting Aleph...\r\n");

    uart0::send(format!("Kernel takes {0} KiB\r\n", (addr_of!(_HEAP_END) as usize - 0x8000_0000) / 1024).as_bytes());

    let mem = MemorySubsystem::new(addr_of!(_HEAP_END) as usize, 0x8000_0000 + 512 * 1024 * 1024);
    uart0::send(b"Created memory SS object\r\n");
    let mut mem_mgr = mem::manager::MEM_MANAGER.get();
    *mem_mgr = Some(mem::manager::MemManager::new());
    mem_mgr.as_mut().unwrap().add_subsystem(mem);
    drop(mem_mgr);
    uart0::send(b"Registered memory SS\r\n");
    let mut kernel_space = mem::virt::VirtualSpace::new();

    let page_count = (addr_of!(_HEAP_END) as usize - 0x8000_0000) / 4096;

    for page in 0..page_count {
        let addr = 0x8000_0000 + page * 4096;
        kernel_space.map_raw(addr, addr);
    }
    uart0::send(b"Mapped kernel memory\r\n");

    let ttbr = kernel_space.table_ptr();
    core::arch::asm!("mcr p15, 0, {}, c2, c0, 0", in(reg) ttbr);
    uart0::send(b"I'm still alive!!\r\n");

    loop {}
}

extern "C" {
    static _IMAGE_END: usize;
    static _HEAP_END: usize;
    static _STACK: usize;
}

#[no_mangle]
extern "C" fn __aeabi_unwind_cpp_pr0() -> ! {
    panic!()
}

#[panic_handler]
fn __panic(_: &core::panic::PanicInfo) -> ! {
    uart0::send(b"Panic occured\r\n");
    loop {}
}
