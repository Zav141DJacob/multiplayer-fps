use once_cell::sync::Lazy;

use common::map::Textured;

use crate::game::texture::sampler::TextureSampler;

pub mod pixels;
pub mod sampler;

pub fn get_wall_texture(tex: Textured) -> &'static TextureSampler {
    match tex {
        Textured::Redstone => &ATLAS_WALL[0],
        Textured::GrayBrick => &ATLAS_WALL[1],
        Textured::RedBrick => &ATLAS_WALL[2],
        Textured::Door => &ATLAS_WALL[3],
        Textured::Green => &ATLAS_WALL[4],
        Textured::Graystone => &ATLAS_WALL[5],
    }
}

pub static ATLAS_WALL: Lazy<Vec<TextureSampler>> = Lazy::new(|| {
    TextureSampler::from_tiles(64, 64, 0, include_bytes!("../../../assets/walltext.png")).unwrap()
});

pub static ATLAS_MONSTER: Lazy<Vec<TextureSampler>> = Lazy::new(|| {
    TextureSampler::from_tiles(64, 64, 0, include_bytes!("../../../assets/monsters.png")).unwrap()
});

pub static TEX_TEST1: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/test1.png").as_slice()).unwrap()
});

pub static TEX_TEST2: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/test2.png").as_slice()).unwrap()
});
