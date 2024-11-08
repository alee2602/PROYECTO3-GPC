use crate::color::Color;
use nalgebra_glm::{Vec2, Vec3};

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub color: Color,
    pub transformed_position: Vec3,
    pub transformed_normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Color::black(),
            transformed_position: position,
            transformed_normal: normal,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color: Color::black(),
            transformed_position: Vec3::new(0.0, 0.0, 0.0),
            transformed_normal: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}
