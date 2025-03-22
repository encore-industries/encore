#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;

use bootloader_api::{BootInfo, entry_point};
use core::panic::PanicInfo;
use vga::{FrameBufferWriter, WRITER};

entry_point!(encore_main);

fn encore_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let info = framebuffer.info();
    let framebuffer = framebuffer.buffer_mut();

    let writer = FrameBufferWriter::new(framebuffer, info);
    *WRITER.lock() = Some(writer);

    let version = "0.1.0";
    println!("Encore OS {}", version);
    println!("Copyright (c) Encore Industries 2025");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
