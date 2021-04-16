#![no_std]
#![no_main]

use::core::panic::PanicInfo;

// We need to define our own panic handler as we can't use the one in
// standard library.

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

static HELLO: &[u8] = b"Hello New OS and Atulu!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}
