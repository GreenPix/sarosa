
use cgmath::Vector2;

pub struct Material {
    stuff: u32,
}

pub struct Object {
    position: Vector2<f32>,
    material: Material,
    //data: RenderData,
}
