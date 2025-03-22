use core::{fmt, panic, ptr::read_volatile};

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use noto_sans_mono_bitmap::{
    FontWeight, RasterHeight, RasterizedChar, get_raster, get_raster_width,
};
use spin::Mutex;

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
pub const CHAR_RASTER: RasterHeight = RasterHeight::Size16;
pub const CHAR_WIDTH: usize = get_raster_width(FONT_WEIGHT, CHAR_RASTER);
pub const CHAR_HEIGHT: usize = CHAR_RASTER.val();

pub static WRITER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);

struct Position {
    x: usize,
    y: usize,
}

pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    position: Position,
}

impl FrameBufferWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut writer = Self {
            framebuffer,
            info,
            position: Position { x: 0, y: 0 },
        };
        writer.clear();

        writer
    }

    fn newline(&mut self) {
        self.position.y += CHAR_HEIGHT + LINE_SPACING;
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.position.x = BORDER_PADDING;
    }

    pub fn clear(&mut self) {
        self.position.x = BORDER_PADDING;
        self.position.y = BORDER_PADDING;

        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    fn load_character_raster(&mut self, c: char) -> RasterizedChar {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER).unwrap()
    }

    pub fn write_char(&mut self, c: char) {
        match c {
            '\r' => self.carriage_return(),
            '\n' => self.newline(),
            c => {
                let new_position = Position {
                    x: self.position.x + CHAR_WIDTH,
                    y: self.position.y + CHAR_HEIGHT + BORDER_PADDING,
                };

                if new_position.x >= self.width() {
                    self.newline();
                }

                if new_position.y >= self.height() {
                    self.clear();
                }

                let raster = self.load_character_raster(c);
                self.render_char(raster);

                // self.position = new_position;
            }
        }
    }

    fn render_char(&mut self, char_raster: RasterizedChar) {
        let raster = char_raster.raster();

        for (y, row) in raster.iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.render_pixel(
                    Position {
                        x: x + self.position.x,
                        y: y + self.position.y,
                    },
                    *byte,
                );
            }
        }

        self.position.x += char_raster.width() + LETTER_SPACING;
    }

    fn render_pixel(&mut self, local_position: Position, intensity: u8) {
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity, 0],
            PixelFormat::Bgr => [intensity, intensity, intensity, 0],
            PixelFormat::U8 => [0, 0, 0, 0],
            _ => {
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("invalid pixel format");
            }
        };

        let pixel_offset = local_position.y * self.info.stride + local_position.x;
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;

        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);

        let _ = unsafe { read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = WRITER.lock();
    if let Some(writer) = writer.as_mut() {
        writer
            .write_fmt(args)
            .expect("VGA framebuffer should be already initialized at encore_main!");
    }
}
