#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_shakyo::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use rust_os_shakyo::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os_shakyo::allocator;
    use rust_os_shakyo::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os_shakyo::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap value at {:?}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os_shakyo::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os_shakyo::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os_shakyo::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}