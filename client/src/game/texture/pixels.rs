use std::ops::{Deref, DerefMut};
use itertools::Itertools;

use notan::draw::{Draw, DrawImages, DrawTransform};
use notan::prelude::{Color, Graphics, Texture};

use crate::helpers::{AsArrays, FlatArrays};

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
        (&mut self.buffer[i..i + 4]).try_into().unwrap()
    }

    /// Clears the pixel buffer with a single color
    pub fn clear(&mut self, color: Color) {
        let rgba = color.rgba_u8();
        self.clear_with(|_, _| rgba)
    }

    /// Clear the pixels with a function that has access to each pixel's X and Y coordinate.
    pub fn clear_with(&mut self, mut f: impl FnMut(usize, usize) -> [u8; 4]) {
        puffin::profile_function!();

        (0..self.width).cartesian_product(0..self.height)
            .for_each(|(x, y)| {
                *self.get_color_u8_mut(x, y) = f(x, y)
            });
    }

    /// Calculate a single column and clear the rest of the pixels with it. The function receives the Y coordinate.
    pub fn clear_with_column(&mut self, f: impl FnMut(usize) -> [u8; 4]) {
        puffin::profile_function!();

        let column: Vec<_> = (0..self.height).map(f).collect();
        let column = column.flat_arrays();
        let len = column.len();

        (0..self.width).for_each(|x| {
            let i = self.xy_to_i(x, 0);
            self.buffer[i..i + len].copy_from_slice(column);
        });
    }

    pub fn column_mut(&mut self, x: usize) -> &mut [[u8; 4]] {
        let i = self.xy_to_i(x, 0);
        self.buffer[i..i + self.height * 4].as_arrays_mut()
    }

    pub fn column_iter_mut(&mut self) -> impl Iterator<Item=&mut [[u8; 4]]> {
        let buffer = self.buffer.as_arrays_mut::<4>();
        buffer.chunks_exact_mut(self.height)
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