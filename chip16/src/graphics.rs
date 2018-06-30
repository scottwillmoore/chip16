const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;

// Each pixel is stored as two 4-bit values in a single byte.
const ADDRESSABLE_VIDEO_MEMORY: usize = SCREEN_WIDTH * SCREEN_HEIGHT / 2;

// Texel might be a better name?
pub type PixelDouble = u8;

pub struct Graphics {
    pub foreground_layer: [u8; ADDRESSABLE_VIDEO_MEMORY],
    pub background_layer: u8,
    pub sprite_width: u8,
    pub sprite_height: u8,
    pub vertical_flip: bool,
    pub horizontal_flip: bool,
    // TODO: Add palette into this struct.
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            foreground_layer: [0; ADDRESSABLE_VIDEO_MEMORY],
            background_layer: 0,
            sprite_width: 0,
            sprite_height: 0,
            vertical_flip: false,
            horizontal_flip: false,
        }
    }

    pub fn clear(&mut self) {
        self.foreground_layer = [0; ADDRESSABLE_VIDEO_MEMORY];
        self.background_layer = 0;
    }

    // These methods may not even be needed.
    // Could be helpful for debugging.
    // pub fn read_pixel(x: u16, y: u16) -> u8 {}
    // pub fn write_pixel(x: u16, y: u16, color: u8) {}

    // pub fn read_pixel_double(x: u16, y: u16) -> u8 {}
    // pub fn write_pixel_double(x: u16, y: u16, pixel_double: u8) {}

    // TODO: Create better variable names.
    // TODO: Make sure this works with signed values. Allow drawing out-of-bounds.
    // pub fn draw_sprite(x_pos: u16, y_pos: u16, sprite_data: &[u8]) {
    //     for x in self.sprite_width {
    //         for y in self.sprite_height {
    //             self.write_pixel_double(i, j, sprite_data[x * width + height]);
    //         }
    //     }
    // }
}
