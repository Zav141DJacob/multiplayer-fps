mod ray_gen;
mod walls;
pub mod sprites;

use glam::Vec2;
use crate::game::raycast::ray_gen::RayGenerator;

const MAX_VIEW_DISTANCE: f32 = 20.0;

pub struct RayCaster {
    ray_gen: RayGenerator,
    proj_dist: f32, // Projection distance
    minimap_rays: Vec<Vec2>,
    depth_map: Vec<f32>,
}

impl RayCaster {
    pub fn new(screen_width: usize, screen_height: usize, fov: f32) -> Self {
        let proj_dist = (screen_height as f32 / 2.0) / (fov.to_radians() / 2.0).tan();
        Self {
            ray_gen: RayGenerator::new(proj_dist, screen_width),
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

    pub fn projection_distance(&self) -> f32 {
        self.proj_dist
    }

    pub fn perspective(&self, angle: f32, camera_height: f32, subject_height: f32) -> Perspective {
        let y_offset = angle.tan() * self.proj_dist;
        let horizon_height = camera_height - subject_height;

        Perspective { y_offset, horizon_height }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Perspective {
    pub y_offset: f32,
    pub horizon_height: f32,
}

impl Perspective {
    pub fn new(angle: f32, camera_height: f32, subject_height: f32, proj_dist: f32) -> Self {
        let y_offset = angle.tan() * proj_dist;
        let horizon_height = camera_height - subject_height;

        Self { y_offset, horizon_height }
    }

    pub fn offset_camera(mut self, by: f32) -> Self {
        self.horizon_height += by;
        self
    }

    pub fn offset_subject(mut self, by: f32, scale: f32) -> Self {
        self.horizon_height -= by / scale;
        self.horizon_height /= scale;
        self
    }
}
