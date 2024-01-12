#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_shakyo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os_shakyo::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_os_shakyo::init();

    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    use rust_os_shakyo::serial;
    use rust_os_shakyo::vga_buffer;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = vga_buffer::WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = vga_buffer::WRITER.lock().buffer.chars[vga_buffer::BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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