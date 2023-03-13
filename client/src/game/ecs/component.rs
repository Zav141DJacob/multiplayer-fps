use crate::game::texture::animated_texture::AnimatedTexture;
use crate::game::texture::sampler::TextureSampler;

/// This entity has had necessary client-side components added
pub struct ClientInitialized;

pub struct RenderSprite {
    pub tex: &'static TextureSampler
}

pub struct ClientSide;