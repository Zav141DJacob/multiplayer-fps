mod ray_gen;
mod walls;
pub mod sprites;

use crate::game::*;
use crate::game::raycast::ray_gen::RayGenerator;


const FOV: f32 = 80.0;
const MAX_VIEW_DISTANCE: f32 = 20.0;
const HORIZON_HEIGHT: f32 = 0.1;
const HEIGHT_OFFSET: f32 = 0.3;

pub struct RayCaster {
    ray_gen: RayGenerator,
    proj_dist: f32, // Projection distance
    minimap_rays: Vec<Vec2>,
    depth_map: Vec<f32>,
}

impl RayCaster {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        let proj_dist = (screen_height as f32 / 2.0) / (FOV.to_radians() / 2.0).tan();
        Self {
            ray_gen: RayGenerator::new(proj_dist, screen_width),
            // ray_gen: RayGenerator::new_old(FOV, screen_width),
            proj_dist,
            minimap_rays: Vec::with_capacity(screen_width),
            depth_map: Vec::with_capacity(screen_width),
        }
    }

    pub fn minimap_rays(&self) -> &[Vec2] {
        &self.minimap_rays
    }

    pub fn depth_map(&self) -> &[f32] {
        &self.depth_map
    }
}



