use crate::geometry::*;

#[derive(Debug)]
pub struct Object {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub triangles: Vec<Triangle3>,
}

impl Object {
    pub fn new() -> Self {
        Object {
            position: Vec3::ORIGIN,
            rotation: Vec3::ORIGIN,
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            triangles: Vec::new(),
        }
    }
}