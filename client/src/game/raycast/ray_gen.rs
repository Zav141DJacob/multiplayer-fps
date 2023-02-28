use glam::Vec2;

/// Caches ray directions for every screen column.
pub struct RayGenerator {
    angles: Vec<Vec2>,
}

impl RayGenerator {
    pub fn new(proj_dist: f32, width: usize) -> Self {
        // Generate angles with forward == (1, 0)

        let half_width = width / 2;

        let start = 0 - half_width as i32;
        let end = (width - half_width) as i32;

        let angles = (start..end)
            .map(|y| Vec2::new(proj_dist, y as f32).normalize())
            .collect();

        Self {
            angles
        }
    }

    pub fn new_old(fov: f32, width: usize) -> Self {
        let step = fov / width as f32;
        let half_fov = fov / 2.0;

        let angles = (0..width)
            .map(|i| -half_fov + step * i as f32)
            .map(|angle| Vec2::from_angle(angle.to_radians()).rotate(Vec2::X))
            .collect();

        Self {
            angles
        }
    }

    pub fn iter(&self, direction: Vec2) -> impl Iterator<Item=Vec2> + '_ {
        self.angles.iter()
            .map(move |&angle| angle.rotate(direction))
    }

    pub fn raw_angles(&self) -> &[Vec2] {
        &self.angles
    }
}