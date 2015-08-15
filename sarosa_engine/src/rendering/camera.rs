use cgmath::Vector2;
use cgmath::Matrix4;
use cgmath::Matrix;

pub struct Camera {
    transform: Matrix4<f32>,
    scale: f32,
}

impl Camera {

    pub fn new() -> Camera {
        Camera {
            transform: Matrix4::identity(),
            scale: 1.0,
        }
    }

    pub fn zoom_in(&mut self) {
        if self.scale < 3.5 {
            self.scale *= 2.0;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.scale > 1.5 {
            self.scale /= 2.0;
        }
    }

    pub fn track(&mut self, position: &Vector2<f32>) {
        let s = self.scale;
        self.transform = Matrix4::new(
              s, 0.0, 0.0, -position.x,
            0.0,   s, 0.0, -position.y,
            0.0, 0.0,   s, 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose();
    }

    pub fn as_uniform(&self) -> &Matrix4<f32> {
        &self.transform
    }

}
