use rendering::scene::MapViewport;
use rendering::scene::MAP_VIEWPORT_UPDATE_RANGE;

use rendering::camera::Camera;

impl MapViewport {

    pub fn update_if_needed(&mut self, camera: &Camera) -> bool {

        let w = camera.width() as f32;
        let h = camera.height() as f32;
        let pos = camera.looking_at();

        let contains =
            pos.x + w / 2.0 < self.max_x() &&
            pos.x - w / 2.0 > self.min_x() &&
            pos.y + h / 2.0 < self.max_y() &&
            pos.y - h / 2.0 > self.min_y();

        if !contains {
            self.x = (pos.x / self.tile_size as f32) as u32;
            self.y = (pos.y / self.tile_size as f32) as u32;
            self.width = (w / self.tile_size as f32) as u32 + MAP_VIEWPORT_UPDATE_RANGE;
            self.height = (h / self.tile_size as f32) as u32 + MAP_VIEWPORT_UPDATE_RANGE;
        }

        !contains
    }


    fn max_x(&self) -> f32 {
        (self.tile_size * self.x + self.tile_size * self.width / 2) as f32
    }

    fn min_x(&self) -> f32 {
        (self.tile_size * self.x - self.tile_size * self.width / 2) as f32
    }

    fn max_y(&self) -> f32 {
        (self.tile_size * self.y + self.tile_size * self.height / 2) as f32
    }

    fn min_y(&self) -> f32 {
        (self.tile_size * self.y - self.tile_size * self.height / 2) as f32
    }
}
