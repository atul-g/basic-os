#![no_std]
#![no_main]

mod vga_buffer;

use::core::panic::PanicInfo;

// We need to define our own panic handler as we can't use the one in
// standard library.

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

//static HELLO: &[u8] = b"Hello New OS and Atulu!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //panic!("random panic message");
    println!("Hello World {}", "!");

    loop{}
}
