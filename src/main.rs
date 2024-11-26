#![no_std] // Don't link Rust std lib
#![no_main] // Disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

// Function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}
