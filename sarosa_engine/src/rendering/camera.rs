use cgmath;
use cgmath::Vector2;
use cgmath::Matrix4;
use cgmath::Matrix;
use num::traits::Zero;
use unit::GAME_UNIT_TO_PX;

pub struct Camera {
    width: u32,
    height: u32,
    pos: Vector2<f32>,
    scale: f32,
}

impl Camera {

    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            pos: Vector2::zero(),
            scale: 1.0,
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> PixelUnit {
        self.width
    }

    pub fn height(&self) -> PixelUnit {
        self.height
    }

    pub fn looking_at(&self) -> Vector2<f32> {
        self.pos
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
        self.pos = position;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn as_uniform(&self) -> Matrix4<f32> {
        Camera::ortho(self.width, self.height) * Camera::modelview(self.pos, self.scale)
    }

    fn modelview(position: &Vector2<f32>, s: f32) -> Matrix4<f32> {
        Matrix4::new(
              s, 0.0, 0.0, - s * position.x * GAME_UNIT_TO_PX,
            0.0,   s, 0.0, - s * position.y * GAME_UNIT_TO_PX,
            0.0, 0.0,   s, 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose()
    }

    fn ortho(width: u32, height: u32) -> Matrix4<f32> {
        let w = width as f32;
        let h = height as f32;
        let m = cgmath::ortho(- w / 2.0, w / 2.0, - h / 2.0, h / 2.0, -h / 2.0, h / 2.0);
        m
    }
}
