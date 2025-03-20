#![no_std]
#![no_main]

use bootloader_api::{BootInfo, entry_point};
use core::{panic::PanicInfo, ptr};
use noto_sans_mono_bitmap::{FontWeight, RasterHeight, get_raster_width};

entry_point!(encore_main);

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

fn encore_main(boot_info: &'static mut BootInfo) -> ! {
    let info = boot_info.framebuffer.as_mut().unwrap().info();
    let framebuffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let mut x_pos: usize = 0;
    let mut y_pos: usize = 0;

    // clean the screen
    framebuffer.fill(0);

    // load noto sans font crate
    let c = 'A';
    let raster =
        noto_sans_mono_bitmap::get_raster(c, FontWeight::Regular, RasterHeight::Size16).unwrap();
    let char = raster.raster();

    let new_x_pos = x_pos + get_raster_width(FontWeight::Regular, RasterHeight::Size16);
    let new_y_pos = y_pos + RasterHeight::Size16.val() + BORDER_PADDING;

    for (y, row) in char.iter().enumerate() {
        for (x, byte) in row.iter().enumerate() {
            let pixel_offset = y * info.stride + x;

            let color = [byte.clone(), byte.clone(), byte / 2, 0];

            let bytes_per_pixel = info.bytes_per_pixel;
            let byte_offset = pixel_offset * bytes_per_pixel;

            framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
                .copy_from_slice(&color[..bytes_per_pixel]);
            let _ = unsafe { ptr::read_volatile(&framebuffer[byte_offset]) };
        }
    }
    x_pos += raster.width() + LETTER_SPACING;

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
