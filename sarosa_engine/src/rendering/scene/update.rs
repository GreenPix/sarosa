use rendering::scene::Vertex;
use rendering::scene::Map;
use rendering::scene::TileLayerWithDepth;
use rendering::scene::TileLayer;
use rendering::camera::Camera;

use Window;
use glium::VertexBuffer;
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{
    PrimitiveType,
    IndexBuffer
};

impl Map {

    pub fn update(&mut self, camera: &Camera, window: &Window) {

        let update_is_needed = self.viewport.update_if_needed(camera);

        if update_is_needed {

            let ref display = window.display;
            let nb_tiles = self.viewport.width * self.viewport.height;

            // Reallocate vertex buffer if not enough element are present
            if self.vertices.len() < nb_tiles * 4 {
                self.vertices = VertexBuffer::empty(display, nb_tiles * 4).unwrap();
                self.update_indices(display, nb_tiles);
            }

            let i_min = self.viewport.x - self.viewport.width / 2;
            let i_max = self.viewport.x + self.viewport.width / 2;
            let j_min = self.viewport.y - self.viewport.height / 2;
            let j_max = self.viewport.y + self.viewport.height / 2;

            assert!((i_max - i_min) * (j_max - j_min) * 4 <= self.vertices.len());

            let mut tile_iter = self.vertices.map().chunks_mut(4);
            let half_tile = self.viewport.tile_size / 2;

            for i in i_min..i_max {
                for j in j_min..j_max {

                    let mut tile = tile_iter.next().unwrap();
                    let position = (i * self.tile_size as f32, j * self.tile_size as f32);

                    tile[0].position[0] = position.0 - half_tile;
                    tile[0].position[1] = position.1 + half_tile;
                    tile[1].position[0] = position.0 + half_tile;
                    tile[1].position[1] = position.1 + half_tile;
                    tile[2].position[0] = position.0 - half_tile;
                    tile[2].position[1] = position.1 - half_tile;
                    tile[3].position[0] = position.0 + half_tile;
                    tile[3].position[1] = position.1 - half_tile;

                }
            }

            for layer in self.layers.iter_mut() {
                layer.update(i_min, i_max, j_min, j_max, window);
            }

        }
    }

    fn update_indices(&mut self, display: &GlutinFacade, nb_tiles: u32) {

        let mut indices = Vec::with_capacity(nb_tiles * 6);

        for num in 0..nb_tiles {

            indices.push(num * 4);
            indices.push(num * 4 + 1);
            indices.push(num * 4 + 2);
            indices.push(num * 4 + 1);
            indices.push(num * 4 + 3);
            indices.push(num * 4 + 2);
        }

        self.indices = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap()
    }
}

impl TileLayerWithDepth {

    fn update(&mut self, i_min: u32, i_max: u32, j_min: u32, j_max: u32, window: &Window) {
        self.0.update(i_min, i_max, j_min, j_max, window);
    }
}

impl TileLayer {

    fn update(&mut self, i_min: u32, i_max: u32, j_min: u32, j_max: u32, window: &Window) {

        let ref display = window.display;

    }
}
