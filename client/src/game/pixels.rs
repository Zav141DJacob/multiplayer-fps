use notan::prelude::{Color, Graphics, Texture};

pub struct Pixels {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    texture: Texture
}

impl Pixels {
    pub fn new(width: usize, height: usize, gfx: &mut Graphics) -> Self {
        let buffer = vec![0; width * height * 4];

        let texture = gfx.create_texture()
            .from_bytes(&buffer, width as i32, height as i32)
            .build()
            .expect("couldn't create pixels texture");

        let mut pixels = Self {
            width,
            height,
            buffer,
            texture,
        };

        pixels.clear(Color::BLACK);

        pixels
    }

    /// Set the color of a single pixel.
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        debug_assert!(x < self.width, "x: {}, width: {}", x, self.width);
        debug_assert!(y < self.height, "y: {}, height: {}", y, self.height);

        let i = (y * self.width + x) * 4;
        self.buffer[i..i+4].copy_from_slice(&color.rgba_u8())
    }

    /// Clears the pixel buffer with a single color
    pub fn clear(&mut self, color: Color) {
        let rgba = color.rgba_u8();
        let mut i = 0;
        while i <= self.buffer.len() - 4 {
            self.buffer[i..i+4].copy_from_slice(&rgba);
            i += 4;
        }
    }

    /// Flushes pixel buffer to texture. Should only be done once per frame.
    pub fn flush(&mut self, gfx: &mut Graphics) {
        gfx.update_texture(&mut self.texture)
            .with_data(&self.buffer)
            .update()
            .expect("couldn't update render texture");
    }

    /// Get the texture for rendering
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Returns (width, height)
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}