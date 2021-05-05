#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(basic_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use basic_rust_os::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use basic_rust_os::memory;
    use basic_rust_os::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::{Page, Translate}, VirtAddr};

    //use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    basic_rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    //mapping an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    //write a string, "New!" to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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

