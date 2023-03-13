use std::cmp::Reverse;
use glam::Vec2;
use ordered_float::OrderedFloat;
use crate::game::raycast::{Perspective, RayCaster};
use crate::game::texture::draw_column::DrawColumn;
use crate::game::texture::pixels::{blend_color_u8, Pixels};
use crate::game::texture::sampler::TextureSampler;
use crate::profile_scope_chain;

/// Stores info about how to render a sprite
pub struct Sprite {
    pub texture: &'static TextureSampler,
    pub position: Vec2,
    pub scale: Vec2,
    pub height_offset: f32,
    distance_2: f32,
}

impl Sprite {
    pub fn new(texture: &'static TextureSampler, position: Vec2, scale: Vec2, height_offset: f32) -> Self {
        let height_offset = height_offset * scale.y;
        Self {
            texture,
            position,
            scale,
            height_offset,
            distance_2: f32::NAN, // Gets overwritten when rendering
        }
    }

    pub fn new_simple(texture: &'static TextureSampler, position: Vec2) -> Self {
        Self::new(texture, position, Vec2::ONE, 0.0)
    }
}

impl RayCaster {
    pub fn draw_sprites(
        &mut self,
        pixels: &mut Pixels,
        camera_pos: Vec2,
        camera_dir: Vec2,
        perspective: Perspective,
        sprites: &mut [Sprite],
    ) {
        puffin::profile_function!();
        debug_assert_eq!(self.depth_map.len(), self.ray_gen.raw_angles().len());

        profile_scope_chain!(start _a, "update distances");
        // Update sprite distances
        sprites.iter_mut().for_each(|sprite| {
            sprite.distance_2 = (sprite.position - camera_pos).length_squared()
        });

        profile_scope_chain!(_a, "sort by distances");
        // Sort so further sprites are first (for painter's algorithm)
        sprites.sort_unstable_by_key(|sprite| Reverse(OrderedFloat(sprite.distance_2)));
        profile_scope_chain!(end _a);

        // We can ignore sprites that are beyond this
        let max_depth_2 = self.depth_map.iter()
            .copied()
            .max_by(|a, b| a.total_cmp(b))
            .expect("Depth map not initialized. Try running draw_walls before draw_sprites?")
            .powi(2);

        // This is used to rotate the perspective so that the camera is facing (1, 0)
        let inverse_camera_rotate = Vec2::new(camera_dir.x, -camera_dir.y);

        // These are the corresponding directions for every screen column with camera_dir at (1, 0)
        let angles = self.ray_gen.raw_angles();

        for sprite in sprites.iter().skip_while(|&sprite| sprite.distance_2 > max_depth_2) {
            puffin::profile_scope!("show sprite");

            let to_sprite = sprite.position - camera_pos;
            let to_sprite = inverse_camera_rotate.rotate(to_sprite);
            let to_sprite_dir = to_sprite / sprite.distance_2.sqrt();

            // Calculate where the sprite's left and right points are relative to the camera
            let perp_norm = to_sprite_dir.perp();
            let right_offset = perp_norm * sprite.scale.x * 0.5;
            let right_most = to_sprite + right_offset;
            let left_most = to_sprite - right_offset;


            // If both points are behind the camera...
            if left_most.x < 0.0 && right_most.x < 0.0 {
                continue
            }

            // Need directions to compare against ray_gen angles.
            // If their x component is <0, then they are behind the camera.
            // Their y component can be compared with the ray_gen angle to see how far across the screen it is
            let left_most_dir = left_most.normalize();
            let right_most_dir = right_most.normalize();

            // ray_gen.angles contains rotation vectors for each screen pixel with camera facing (1, 0)
            let left_most_i = if left_most_dir.x < 0.0 {
                0
            } else {
                // Find what angle hit to the right of the left most point
                angles.partition_point(|angle| angle.y < left_most_dir.y)
            };

            let right_most_i = if right_most_dir.x < 0.0 {
                angles.len()
            } else {
                // Find what angles hit to the left of the right most point
                angles.partition_point(|angle| angle.y <= right_most_dir.y)
            };

            // println!("left: {}, right: {}", left_most_i, right_most_i);

            // These are all the angles that hit out sprite
            let valid_angles = &angles[left_most_i..right_most_i];

            // This is used to rotate the perspective so that the sprite's center is facing (1, 0)
            let inverse_to_sprite_dir = Vec2::new(to_sprite_dir.x, -to_sprite_dir.y);
            // This is equal to (to_sprite.length(), 0)
            let to_sprite_axis_aligned = inverse_to_sprite_dir.rotate(to_sprite);

            let perspective = perspective.offset_subject(sprite.height_offset, sprite.scale.y);

            profile_scope_chain!(start _a, "iterate visible angles");
            for (screen_x, angle) in (left_most_i..right_most_i).zip(valid_angles) {
                // Rotate angle to a perspective where to_sprite_dir is (1, 0)
                let angle_rot = inverse_to_sprite_dir.rotate(*angle);
                // This is how long the ray to this particular point on the sprite is
                let this_angle_len = to_sprite_axis_aligned.x / angle_rot.x;

                // Check occlusion
                if this_angle_len > self.depth_map[screen_x] {
                    continue
                }

                let column_height = sprite.scale.y * self.proj_dist / (this_angle_len * angle.dot(Vec2::X));

                // This is what this angle's hit Y component would be if to_sprite_dir is at (1, 0)
                let hit_y = angle_rot.y * this_angle_len;
                // Adjust hit_y to -0.5..0.5 and add 0.5
                let tex_x = hit_y / sprite.scale.x + 0.5;

                sprite.texture.draw_column(
                    pixels.column_mut(screen_x),
                    tex_x,
                    column_height,
                    perspective,
                    blend_color_u8,
                )
            };
        }
    }
}
