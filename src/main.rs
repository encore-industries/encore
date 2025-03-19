#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::{BootInfo, entry_point};

entry_point!(encore_main);

fn encore_main(_boot_info: &'static mut BootInfo) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
