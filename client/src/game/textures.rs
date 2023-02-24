use notan::prelude::Color;
use std::io::Cursor;

pub struct Texture {
    pub count: usize,
    pub size: usize,
    pub texture: Vec<Color>,
}

impl Texture {
    pub fn new(bytes: &[u8]) -> anyhow::Result<Self> {
        let textures = image::io::Reader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?
            .into_rgba8();
        let (width, height) = textures.dimensions();

        let count = width / height;
        let size = width / count;
        let mut texture = vec![Color::new(0., 0., 0., 0.); (width * height) as usize];
        let v = textures.into_vec();
        for j in 0..height {
            for i in 0..width {
                let r = v[((i + j * width) * 4) as usize] as f32 / 255.;
                let g = v[((i + j * width) * 4 + 1) as usize] as f32 / 255.;
                let b = v[((i + j * width) * 4 + 2) as usize] as f32 / 255.;
                let a = v[((i + j * width) * 4 + 3) as usize] as f32 / 255.;
                texture[(i + j * width) as usize] = Color::new(r, g, b, a);
            }
        }
        Ok(Texture {
            count: count as usize,
            size: size as usize,
            texture,
        })
    }
    pub fn get(&self, i: i32, j: i32, idx: i32) -> Color {
        self.texture[(i + idx * self.size as i32 + j * (self.size * self.count) as i32) as usize]
    }

    pub fn texture_column(&self, texid: i32, texcoord: f32, column_height: f32) -> Vec<Color> {
        let img_w = self.size * self.count;
        let mut tex = vec![Color::new(0.0, 0.0, 0.0, 0.); column_height as usize];
        for i in 0..column_height as usize {
            let pix_x = texid as usize * self.size + texcoord as usize;
            let pix_y = (i * self.size) / column_height as usize;
            if (pix_x + pix_y * img_w) >= self.texture.len() {
                return tex;
            }
            tex[i] = self.texture[pix_x + pix_y * img_w];
        }
        tex
    }
}
