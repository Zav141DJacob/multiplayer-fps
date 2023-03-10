
use once_cell::sync::Lazy;

use common::map::Textured;
use crate::game::texture::animated_texture::AnimatedTexture;


use crate::game::texture::sampler::TextureSampler;

pub mod pixels;
pub mod sampler;
pub mod draw_column;
pub mod animated_texture;

pub fn get_wall_texture(tex: Textured) -> &'static TextureSampler {
    // return &TEX_TEST2;
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
    TextureSampler::from_tiles(6, 1, 0, include_bytes!("../../../assets/walltext.png")).unwrap()
});

pub static ATLAS_MONSTER: Lazy<Vec<TextureSampler>> = Lazy::new(|| {
    TextureSampler::from_tiles(4, 1, 0, include_bytes!("../../../assets/monsters.png")).unwrap()
});

pub static TEX_TEST1: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/test1.png").as_slice()).unwrap()
});

pub static TEX_TEST2: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/test2.png").as_slice()).unwrap()
});

pub static ATLAS_PLAYER: Lazy<Vec<TextureSampler>> = Lazy::new(|| {
    TextureSampler::from_tiles(8, 7, 1, include_bytes!("../../../assets/player.png")).unwrap()
});

pub static ANIMATED_PLAYER: Lazy<AnimatedTexture> = Lazy::new(|| {
    AnimatedTexture::new(&ATLAS_PLAYER)
        .register_state("standing", 0.5, vec![
        vec![0, 1, 2, 3, 4, 5, 6, 7],
    ])
        .register_state("running", 0.15, vec![
            vec![8, 9, 10, 11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20, 21, 22, 23],
            vec![24, 25, 26, 27, 28, 29, 30, 31],
            vec![32, 33, 34, 35, 36, 37, 38, 39],
        ])
        .register_state("shooting", 0.1, vec![
            vec![48],
            vec![49],
            vec![50],
        ])
});