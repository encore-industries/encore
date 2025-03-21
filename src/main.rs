#![no_std]
#![no_main]

mod vga;

use bootloader_api::{BootInfo, entry_point};
use core::{fmt::Write, panic::PanicInfo};
use vga::FrameBufferWriter;

entry_point!(encore_main);

fn encore_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let info = framebuffer.info();
    let framebuffer = framebuffer.buffer_mut();

    let mut writer = FrameBufferWriter::new(framebuffer, info);
    writer.clear();
    writer
        .write_str("Hello world!\nWelcome to encore OS\nversion: 0.1.0")
        .unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
