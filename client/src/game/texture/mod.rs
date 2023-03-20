
use once_cell::sync::Lazy;

use common::map::Textured;
use crate::game::texture::animated_texture::AnimatedTexture;


use crate::game::texture::sampler::TextureSampler;

pub mod pixels;
pub mod sampler;
pub mod draw_column;
pub mod animated_texture;

pub fn get_wall_texture(tex: Textured) -> &'static TextureSampler {
    match tex {
        Textured::Brick1 => &TEX_BRICK1,
        Textured::Brick2 => &TEX_BRICK2,
        Textured::Door => &TEX_DOOR,
        Textured::Industrial => &TEX_INDUSTRIAL,
        Textured::Rocky => &TEX_ROCKY,
        Textured::Techy => &TEX_TECHY,
        Textured::Urban => &TEX_URBAN,
        Textured::Wood => &TEX_WOOD,
    }
}


pub static TEX_BRICK1: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/walls/bricks/ROUNDBRICKS.png").as_slice()).unwrap()
});

pub static TEX_BRICK2: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/walls/bricks/REDBRICKS.png").as_slice()).unwrap()
        .set_dominant([147, 42, 32, 255])
});

pub static TEX_DOOR: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/walls/doors/CREAKYDOOR.png").as_slice()).unwrap()
});

pub static TEX_INDUSTRIAL: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/walls/industrial/CROSSCUBE.png").as_slice()).unwrap()
});

pub static TEX_ROCKY: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/walls/rocks/GRAYROCKS.png").as_slice()).unwrap()
});

pub static TEX_TECHY: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/walls/tech/HEXAGONS.png").as_slice()).unwrap()
});

pub static TEX_URBAN: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/walls/urban/GRAYWALL.png").as_slice()).unwrap()
});

pub static TEX_WOOD: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/walls/wood/WOODA.png").as_slice()).unwrap()
});


// pub static ATLAS_WALL: Lazy<Vec<TextureSampler>> = Lazy::new(|| {
//     TextureSampler::from_tiles(6, 1, 0, include_bytes!("../../../assets/walltext.png")).unwrap()
// });
//
// pub static ATLAS_MONSTER: Lazy<Vec<TextureSampler>> = Lazy::new(|| {
//     TextureSampler::from_tiles(4, 1, 0, include_bytes!("../../../assets/monsters.png")).unwrap()
// });

pub static WEAPON_CRATE: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/weapon_crate.png").as_slice()).unwrap()
});

pub static TEX_TEST1: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/test1.png").as_slice()).unwrap()
});

pub static TEX_TEST2: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/test2.png").as_slice()).unwrap()
});

pub static TEX_BULLET: Lazy<TextureSampler> = Lazy::new(|| {
    TextureSampler::try_from(include_bytes!("../../../assets/bullet.png").as_slice()).unwrap()
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

pub static ANIMATED_DEATH: Lazy<AnimatedTexture> = Lazy::new(|| {
    AnimatedTexture::new(&ATLAS_PLAYER)
        .register_state("dying", 0.4, vec![
            vec![40],
            vec![41],
            vec![42],
            vec![43],
            vec![44],
        ])
});