use cgmath;
use cgmath::Vector2;
use cgmath::Matrix4;
use cgmath::Matrix;
use unit::GAME_UNIT_TO_PX;

pub struct Camera {
    transform: Matrix4<f32>,
    projection: Matrix4<f32>,
    scale: f32,
}

impl Camera {

    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            transform: Matrix4::identity(),
            scale: 1.0,
            projection: Camera::ortho(width, height),
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
              s, 0.0, 0.0, - s * position.x * GAME_UNIT_TO_PX,
            0.0,   s, 0.0, - s * position.y * GAME_UNIT_TO_PX,
            0.0, 0.0,   s, 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose();
    }

    pub fn sdflkj() {
        self.projection = Window::ortho(width, height);
    }

    pub fn as_uniform(&self) -> &Matrix4<f32> {
        &self.transform
    }

    fn ortho(width: u32, height: u32) -> Matrix4<f32> {
        let w = width as f32;
        let h = height as f32;
        let m = cgmath::ortho(- w / 2.0, w / 2.0, - h / 2.0, h / 2.0, -1.0, 1.0);
        m
    }
}
