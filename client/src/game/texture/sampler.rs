use std::ops::Range;

use anyhow::bail;
use image::{imageops, RgbaImage};

pub struct TextureSampler {
    /// IMPORTANT: This is a transposed version of the original image.
    /// Doing it like this increases CPU cache performance when reading columns.
    /// This only affects the private implementation, not the public API.
    image: RgbaImage,

    /// The dominant color of this texture, for use in minimap.
    dominant: [u8; 4],
}

impl TryFrom<&[u8]> for TextureSampler {
    type Error = image::ImageError;

    /// Create a texture from rgba bytes
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut image = image::load_from_memory(bytes)?.into_rgba8();

        Ok(Self::from(&image))
    }
}

impl From<&RgbaImage> for TextureSampler {
    /// Create a texture from an existing image
    fn from(image: &RgbaImage) -> Self {
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
            image,
            dominant,
        }
    }
}

impl TextureSampler {
    /// Turns a given image into a vector of GameTextures. Takes in the width and height of a single tile in the image.
    /// Might be useful for animations and texture atlases.
    pub fn from_tiles(tile_width: u32, tile_height: u32, gap: u32, bytes: &[u8]) -> anyhow::Result<Vec<Self>> {
        let big_image = image::load_from_memory(bytes)?.into_rgba8();

        if big_image.width() % tile_width != 0 {
            bail!("image width {} is not an exact multiple of tile width {}", big_image.width(), tile_width);
        }
        if big_image.height() % tile_height != 0 {
            bail!("image height {} is not an exact multiple of tile height {}", big_image.height(), tile_height);
        }

        let res = (0..(big_image.width() / tile_width)).flat_map(|x| {
            (0..(big_image.width() / tile_width)).map(|y| (x, y))
        })
            .map(|(tile_x, tile_y)| {
                let x = tile_x * (tile_width + gap);
                let y = tile_y * (tile_height + gap);
                imageops::crop_imm(&big_image, x, y, tile_width, tile_height).to_image()
            })
            .map(|img| TextureSampler::from(&img))
            .collect();

        Ok(res)
    }

    /// Get the dominant color of this texture
    pub fn dominant(&self) -> [u8; 4] {
        self.dominant
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
        let x = (u * self.image.height() as f32) as i32;
        let y = (v * self.image.width() as f32) as i32;
        self.sample_exact(x, y)
    }

    /// Sample a color based on exact pixel coordinates
    pub fn sample_exact(&self, x: i32, y: i32) -> [u8; 4] {
        let x = x.rem_euclid(self.image.height() as i32) as u32;
        let y = y.rem_euclid(self.image.width() as i32) as u32;
        let px = self.image.get_pixel(y, x);
        px.0
    }

    /// Samples a given number of colors in column `u` according to a range of `v`.
    ///
    /// Here are some pseudocode examples:
    /// ```rust,no-run
    /// let tex = GameTexture::from(img); // Single-column texture with colors ABCD
    /// tex.sample_column(0, 0.0..1.0, 4) == ABCD;
    /// tex.sample_column(0, 0.0..0.5, 4) == AABB;
    /// tex.sample_column(0, 0.5..1.0, 4) == CCDD;
    /// tex.sample_column(0, 0.0..1.0, 8) == AABBCCDD;
    /// tex.sample_column(0, 0.0..1.0, 2) == AC;
    /// tex.sample_column(0, -1.0..2.0, 12) == ABCDABCDABCD;
    /// ```
    pub fn sample_column(&self, u: f32, v_range: Range<f32>, height: usize) -> impl Iterator<Item=[u8; 4]> + DoubleEndedIterator + '_ {
        let x = (u * self.image.height() as f32) as i32;
        let y_start = (v_range.start * self.image.width() as f32) as i32;
        let y_end = (v_range.end * self.image.width() as f32) as i32;

        self.sample_column_exact(x, y_start..y_end, height)
    }

    /// Same as [Self::sample_column], except it uses exact pixel coordinates.
    pub fn sample_column_exact(&self, x: i32, y_range: Range<i32>, height: usize) -> impl Iterator<Item=[u8; 4]> + DoubleEndedIterator + '_ {
        let y_start = y_range.start as f32;
        let y_end = y_range.end as f32;

        (0..height).map(move |h| {
            let fraction = h as f32 / height as f32;
            let y = lerp(y_start, y_end, fraction) as i32;
            self.sample_exact(x, y)
        })
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}