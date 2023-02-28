use std::ops::{Deref, DerefMut};

use notan::draw::{Draw, DrawImages, DrawTransform};
use notan::prelude::{Color, Graphics, Texture};

pub struct Pixels {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    texture: Texture,
}

impl Pixels {
    pub fn new(width: usize, height: usize, gfx: &mut Graphics) -> Self {
        let buffer = vec![0; width * height * 4];

        let texture = gfx
            .create_texture()
            .from_bytes(&buffer, height as i32, width as i32)
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

    fn xy_to_i(&self, x: usize, y: usize) -> usize {
        (x * self.height + y) * 4
    }

    fn i_to_xy(&self, i: usize) -> (usize, usize) {
        (i / (self.height * 4), i % (self.height * 4) / 4)
    }

    /// Set the color of a single pixel.
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        self.set_color_u8(x, y, color.rgba_u8())
    }

    /// Set the color of a single pixel.
    pub fn set_color_u8(&mut self, x: usize, y: usize, color: [u8; 4]) {
        debug_assert!(x < self.width, "x: {}, width: {}", x, self.width);
        debug_assert!(y < self.height, "y: {}, height: {}", y, self.height);

        let i = self.xy_to_i(x, y);
        self.buffer[i..i + 4].copy_from_slice(&color)
    }

    pub fn blend_color(&mut self, x: usize, y: usize, color: Color) {
        self.blend_color_u8(x, y, color.rgba_u8())
    }

    pub fn blend_color_u8(&mut self, x: usize, y: usize, color: [u8; 4]) {
        blend_color_u8(self.get_color_u8_mut(x, y), color)
    }


    /// Returns a closure that modifies a color before blending it.
    pub fn blend_color_u8_with(mut f: impl FnMut([u8; 4]) -> [u8; 4]) -> impl FnMut(&mut Self, usize, usize, [u8; 4]) {
        move |this, x, y, color| {
            let color = f(color);
            this.blend_color_u8(x, y, color)
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.get_color_u8(x, y).into()
    }

    pub fn get_color_u8(&self, x: usize, y: usize) -> [u8; 4] {
        let i = self.xy_to_i(x, y);
        self.buffer[i..i + 4].try_into().unwrap()
    }

    pub fn get_color_u8_mut(&mut self, x: usize, y: usize) -> &mut [u8; 4] {
        let i = self.xy_to_i(x, y);
        // Should be safe since we're always gonna get a slice with length 4
        unsafe { u8_to_rgba_one_mut(&mut self.buffer[i..i + 4]) }
    }

    /// Clears the pixel buffer with a single color
    pub fn clear(&mut self, color: Color) {
        let rgba = color.rgba_u8();
        self.clear_with(|_, _| rgba)
    }

    pub fn clear_with(&mut self, mut f: impl FnMut(usize, usize) -> [u8; 4]) {
        puffin::profile_function!();
        let mut i = 0;
        while i <= self.buffer.len() - 4 {
            let (x, y) = self.i_to_xy(i);
            let color = f(x, y);
            self.buffer[i..i + 4].copy_from_slice(&color);
            i += 4;
        }
    }

    /// Flushes pixel buffer to texture. Should only be done once per frame.
    pub fn flush(&mut self, gfx: &mut Graphics) {
        puffin::profile_function!();

        gfx.update_texture(&mut self.texture)
            .with_data(&self.buffer)
            .update()
            .expect("couldn't update render texture");
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns (width, height)
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn draw<'a>(&'a self, draw: &'a mut Draw) -> PixelsDraw<'a> {
        PixelsDraw(draw.image(&self.texture))
    }
}

type DrawBuilder<'a> = notan::draw::DrawBuilder<'a, notan::draw::Image<'a>>;

pub struct PixelsDraw<'a> (DrawBuilder<'a>);

impl<'a> Drop for PixelsDraw<'a> {
    fn drop(&mut self) {
        use notan::math::{Mat3, Vec3};
        let transpose_mat = Mat3::from_cols(Vec3::Y, Vec3::X, Vec3::Z);
        let old = self.0.matrix().unwrap_or(Mat3::IDENTITY);
        self.0.matrix().replace(old * transpose_mat);
    }
}

impl<'a> Deref for PixelsDraw<'a> {
    type Target = DrawBuilder<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for PixelsDraw<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


// TODO: Move some of the stuff below here into another file maybe?

// SAFETY: These should be safe to use because both types are stored the same way in memory and are aligned to 1 byte.
// Worst case, u8_to_rgba will return a shorter slice if length is not 4 byte aligned.
pub fn u8_to_rgba_mut(slice_u8: &mut [u8]) -> &mut [[u8; 4]] {
    unsafe {
        use std::slice;
        slice::from_raw_parts_mut(slice_u8.as_mut_ptr() as *mut [u8; 4], slice_u8.len() / 4)
    }
}

pub fn u8_to_rgba(slice_u8: &[u8]) -> &[[u8; 4]] {
    unsafe {
        use std::slice;
        slice::from_raw_parts(slice_u8.as_ptr() as *const [u8; 4], slice_u8.len() / 4)
    }
}

pub fn rgba_to_u8(slice_rgba: &[[u8; 4]]) -> &[u8] {
    unsafe {
        use std::slice;
        slice::from_raw_parts(slice_rgba.as_ptr() as *const u8, slice_rgba.len() * 4)
    }
}

pub fn rgba_to_u8_mut(slice_rgba: &mut [[u8; 4]]) -> &mut [u8] {
    unsafe {
        use std::slice;
        slice::from_raw_parts_mut(slice_rgba.as_mut_ptr() as *mut u8, slice_rgba.len() * 4)
    }
}

/// SAFETY: Only safe if slice is 4 long.
unsafe fn u8_to_rgba_one_mut(slice_u8: &mut [u8]) -> &mut [u8; 4] {
    debug_assert!(slice_u8.len() == 4);
    let ptr = slice_u8.as_mut_ptr() as *mut [u8; 4];
    &mut *ptr
}


#[cfg(not(feature = "semitransparency"))]
pub fn blend_color_u8(back: &mut [u8; 4], front: [u8; 4]) {
    if front[3] == 0 {
        return;
    }

    *back = front
}

#[cfg(feature = "semitransparency")]
pub fn blend_color_u8(back: &mut [u8; 4], front: [u8; 4]) {
    if front[3] == 0 {
        return;
    }

    if front[3] == 255 {
        *back = front;
        return;
    }

    fn lerp(start: u8, end: u8, t: u8) -> u8 {
        ((start as u16 * (255 - t) as u16 + end as u16 * t as u16) / 255) as u8
    }

    back[0] = lerp(back[0], front[0], front[3]);
    back[1] = lerp(back[1], front[1], front[3]);
    back[2] = lerp(back[2], front[2], front[3]);
    back[3] = 255 - ((255 - front[3]) * (255 - back[3]) / 255);
}