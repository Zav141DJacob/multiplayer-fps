use std::ops::Range;

use anyhow::bail;
use image::{GenericImageView, imageops, RgbaImage};
use itertools::Itertools;
use crate::helpers::AsArrays;

pub struct TextureSampler {
    /// IMPORTANT: This is a transposed version of the original image.
    /// Doing it like this increases CPU cache performance when reading columns.
    /// This only affects the private implementation, not the public API.
    image: RgbaImage,

    // The un-transpose dimensions
    width: u32,
    height: u32,

    /// The dominant color of this texture, for use in minimap.
    dominant: [u8; 4],
}

impl TryFrom<&[u8]> for TextureSampler {
    type Error = image::ImageError;

    /// Create a texture from rgba bytes
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let image = image::load_from_memory(bytes)?.into_rgba8();

        Ok(Self::from(&image))
    }
}

impl From<&RgbaImage> for TextureSampler {
    /// Create a texture from an existing image
    fn from(image: &RgbaImage) -> Self {
        assert_ne!(image.width(), 0);
        assert_ne!(image.height(), 0);

        // Transpose image (i.e. switch columns and rows)
        let mut image = imageops::rotate90(image);
        imageops::flip_horizontal_in_place(&mut image);

        // Calculate dominant color
        let palette = color_thief::get_palette(image.as_raw(), color_thief::ColorFormat::Rgba, 1, 5)
            .expect("color_thief panicked with valid image input");
        let dominant = palette.first()
            .expect("color_thief didn't return any colors");

        let dominant = [dominant.r, dominant.g, dominant.b, 255];

        Self {
            width: image.height(),
            height: image.width(),
            image,
            dominant,
        }
    }
}

impl TextureSampler {
    /// Turns a given image into a vector of GameTextures. Takes in the width and height of a single tile in the image.
    /// Might be useful for animations and texture atlases.
    pub fn from_tiles(tiles_x: u32, tiles_y: u32, gap: u32, bytes: &[u8]) -> anyhow::Result<Vec<Self>> {
        assert!(tiles_x > 0 && tiles_y > 0, "tiles_x or tiles_y can't be 0");

        let big_image = image::load_from_memory(bytes)?.into_rgba8();

        let gap_x = (tiles_x - 1) * gap;
        let gap_y = (tiles_y - 1) * gap;

        if (big_image.width() - gap_x) % tiles_x != 0 {
            bail!("image width {} is not an exact multiple of tiles_x {}", big_image.width(), tiles_x);
        }
        if (big_image.height() - gap_y) % tiles_y != 0 {
            bail!("image height {} is not an exact multiple of tiles_y {}", big_image.height(), tiles_y);
        }

        let tile_width = big_image.width() / tiles_x;
        let tile_height = big_image.height() / tiles_y;

        let tile_x_range = 0..(big_image.width() / tile_width);
        let tile_y_range = 0..(big_image.height() / tile_height);

        let res = tile_y_range.cartesian_product(tile_x_range)
            .map(|(tile_y, tile_x)| {
                let x = tile_x * (tile_width + gap);
                let y = tile_y * (tile_height + gap);
                imageops::crop_imm(&big_image, x, y, tile_width, tile_height).to_image()
            })
            .map(|img| TextureSampler::from(&img))
            .collect();

        Ok(res)
    }

    pub fn set_dominant(mut self, color: [u8; 4]) -> Self {
        self.dominant = color;
        self
    }

    /// Get the dominant color of this texture
    pub fn dominant(&self) -> [u8; 4] {
        self.dominant
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Gets the original image of this texture.
    /// WARNING: This is an expensive operation because the image needs to be re-transposed first.
    pub fn original_image(&self) -> RgbaImage {
        let mut image = imageops::rotate90(&self.image);
        imageops::flip_horizontal_in_place(&mut image);
        image
    }

    /// Sample a color based on uv coordinates.
    /// NOTE: If you're unfamiliar, uv coordinates are just xy coordinates in the range 0.0..1.0
    pub fn sample(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u * self.width as f32) as i32;
        let y = (v * self.height as f32) as i32;
        self.sample_exact(x, y)
    }

    /// Sample a color based on exact pixel coordinates
    pub fn sample_exact(&self, x: i32, y: i32) -> [u8; 4] {
        // SAFETY: Clamp x and y between 0..width and 0..height respectively, so the unsafe block should be safe.
        let x = x.rem_euclid(self.width as i32) as u32;
        let y = y.rem_euclid(self.height as i32) as u32;
        unsafe { self.sample_exact_unchecked(x, y) }
    }

    /// Only safe to use if x < width and y < height
    pub unsafe fn sample_exact_unchecked(&self, x: u32, y: u32) -> [u8; 4] {
        let px = self.image.unsafe_get_pixel(y, x);
        px.0
    }

    /// Samples a given number of colors in column `u` according to a range of `v`.
    ///
    /// Here are some pseudocode examples:
    /// ```rust,no_run
    /// let tex = GameTexture::from(img); // Single-column texture with colors ABCD
    /// tex.sample_column(0, 0.0..1.0, 4) == ABCD;
    /// tex.sample_column(0, 0.0..0.5, 4) == AABB;
    /// tex.sample_column(0, 0.5..1.0, 4) == CCDD;
    /// tex.sample_column(0, 0.0..1.0, 8) == AABBCCDD;
    /// tex.sample_column(0, 0.0..1.0, 2) == AC;
    /// tex.sample_column(0, -1.0..2.0, 12) == ABCDABCDABCD;
    /// ```
    pub fn sample_column(&self, u: f32, v_range: Range<f32>, height: usize) -> impl Iterator<Item=[u8; 4]> + DoubleEndedIterator + ExactSizeIterator + '_ {
        let x = (u * self.width as f32) as i32;
        let y_start = v_range.start * self.height as f32;
        let y_end = v_range.end * self.height as f32;

        self.sample_column_internal(x, y_start, y_end, height)
    }

    /// Same as [Self::sample_column], except it uses exact pixel coordinates.
    pub fn sample_column_exact(&self, x: i32, y_range: Range<i32>, height: usize) -> impl Iterator<Item=[u8; 4]> + DoubleEndedIterator + ExactSizeIterator + '_ {
        let y_start = y_range.start as f32;
        let y_end = y_range.end as f32;

        self.sample_column_internal(x, y_start, y_end, height)
    }

    fn sample_column_internal(&self, x: i32, y_start: f32, y_end: f32, height: usize) -> impl Iterator<Item=[u8; 4]> + DoubleEndedIterator + ExactSizeIterator + '_ {
        let x = (x as u32).rem_euclid(self.width); // x < self.width

        let col_start = (x * self.height * 4) as usize;
        let col_end = col_start + self.height as usize * 4;
        let column = &self.image.as_raw()[col_start..col_end];
        let column = column.as_arrays::<4>();
        debug_assert_eq!(column.len(), self.height as usize);

        // Make y_start 0..self.height
        let y_offset = y_start.div_euclid(self.height as f32);
        let y_start = y_start + y_offset * self.height as f32;
        let y_end = y_end + y_offset * self.height as f32;

        let mut y_tex = y_start;
        let step = (y_end - y_start) / height as f32;

        (0..height).map(move |_| {
            let y = y_tex as usize;
            y_tex += step;
            let y = y % self.height as usize; // y < self.height

            column[y]
        })
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}
