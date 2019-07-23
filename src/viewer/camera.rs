use crate::geometry::*;

pub struct Camera {
    position: Vec3,
    rotation: Vec3,
    window_dimensions: Vec2,
}

impl Camera {
    pub fn new(dimensions: Vec2) -> Self {
        Camera {
            position: Vec3::new(),
            rotation: Vec3::new(),
            window_dimensions: dimensions,
        }
    }

    pub fn project_vec(&self, v3: &Vec3) -> Vec2 {
        Vec2 {
            x: self.window_dimensions.x / 2.0
                + (v3.x + self.position.x) / (v3.z + self.position.z) * 250.0,
            y: self.window_dimensions.y / 2.0
                - (v3.y + self.position.y) / (v3.z + self.position.z) * 250.0,
        }
    }
}
