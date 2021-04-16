#![no_std]
#![no_main]

use::core::panic::PanicInfo;

// We need to define our own panic handler as we can't use the one in
// standard library.

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}
