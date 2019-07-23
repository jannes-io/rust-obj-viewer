use crate::geometry::*;

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    window_dimensions: Vec2,
}

impl Camera {
    pub fn new(dimensions: Vec2) -> Self {
        Camera {
            position: Vec3::ORIGIN,
            rotation: Vec3::ORIGIN,
            window_dimensions: dimensions,
        }
    }

    pub fn translate(&mut self, v3: Vec3) {
        self.position.x += (self.rotation.y + std::f64::consts::FRAC_PI_2).sin() * v3.x
            + self.rotation.y.sin() * v3.z;
        self.position.y += v3.y;
        self.position.z += -self.rotation.y.cos() * v3.z + self.rotation.y.sin() * v3.x;
    }

    pub fn rotate(v: &Vec3, t: &Vec3) -> Vec3 {
        let mut rot = Vec3 {
            x: v.x,
            y: t.x.cos() * v.y + t.x.sin() * v.z,
            z: -t.x.sin() * v.y + t.x.cos() * v.z,
        };
        rot = Vec3 {
            x: t.y.cos() * rot.x + t.y.sin() * rot.z,
            y: rot.y,
            z: -t.y.sin() * rot.x + t.y.cos() * rot.z,
        };
        Vec3 {
            x: t.z.cos() * rot.x - t.z.sin() * rot.y,
            y: t.z.sin() * rot.x + t.z.cos() * rot.y,
            z: rot.z,
        }
    }

    pub fn offset(&self, v3: Vec3) -> Vec3 {
        self.position + v3
    }

    pub fn project_vec(&self, v3: &Vec3) -> Vec2 {
        let rotated = Self::rotate(&(*v3 + self.position), &self.rotation) - self.position;

        Vec2 {
            x: self.window_dimensions.x / 2.0
                + (rotated.x + self.position.x) / (rotated.z + self.position.z) * 900.0,
            y: self.window_dimensions.y / 2.0
                - (rotated.y + self.position.y) / (rotated.z + self.position.z) * 900.0,
        }
    }
}
