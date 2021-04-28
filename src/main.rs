#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(basic_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use basic_rust_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    basic_rust_os::init();


    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at {:?}", level_4_page_table.start_address());

    //fn stack_overflow() {
    //    stack_overflow(); // for each recursion, the return address is pushed
    //}
    // uncomment line below to trigger a stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    // provoking a deadlock and testing it works
    //loop {
    //    use basic_rust_os::print;
    //    for _ in 0..10000 {}
    //    print!("-");
    //}
    basic_rust_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    basic_rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    basic_rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

