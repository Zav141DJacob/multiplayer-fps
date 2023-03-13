use glam::Vec2;
use crate::game::texture::sampler::TextureSampler;

/// This entity has had necessary client-side components added
pub struct ClientInitialized;

pub struct RenderSprite {
    pub tex: &'static TextureSampler
}

pub struct Scale (pub Vec2);
pub struct Height (pub f32);

pub struct ClientSide;