extern crate rand;

use self::rand::distributions::{IndependentSample, Range};
use image;
use cgmath::Matrix4;
use glium::Surface;
use glium::program::Program;
use glium::index::{
    PrimitiveType,
    IndexBuffer
};
use glium::draw_parameters::DrawParameters;
use glium::texture::Texture2dArray;
use glium::Frame;

use models::game::GameData;
use animation::TextureId;
use rendering::renderer::shaders;
use Window;
use unit::GAME_UNIT_TO_PX;

pub struct MapRenderer {
    program: Program,
    texture: Texture2dArray,
}

// TODO(Nemikolh): Clean up that (should be read from a map file format)
// Same should apply for the FRAMES_PER_TEXTURE constant in animation/mod.rs
const TILES_PER_TEXTURE:u32 = 30 * 16;

impl MapRenderer {

    pub fn new(window: &Window) -> MapRenderer {

        let ref display = window.display;

        let texture = {
            // TODO(Nemikolh): Use a ResourceManager to load
            // them before being here and do something clever with it.
            let images = vec![
                image::open("./assets/maps/tiles.png").unwrap(),
            ];

            Texture2dArray::new(display, images).unwrap()
        };

        let program = program!(display,
            140 => {
                vertex: shaders::map::VERTEX_140,
                fragment: shaders::map::FRAGMENT_140
            },
        ).unwrap();

        MapRenderer {
            program: program,
            texture: texture,
            vertex_buffer: VertexBuffer::empty(display, 0).unwrap(),
            index_buffer: IndexBuffer::empty(display, PrimitiveType::TrianglesList, 0).unwrap(),
        }
    }

    pub fn initialize_gpu_mem(&mut self, game_data: &GameData, window: &Window) {

        let ref display = window.display;
        let width = game_data.get_map().width();
        let height = game_data.get_map().height();
        let TextureId(tex_id) = game_data.get_map().tex_id();
        let nb_tiles = (width * height) as usize;

        let (vertex_buffer, index_buffer) = {
            let mut vb: VertexBuffer<shaders::map::Vertex> =
                VertexBuffer::empty(display, nb_tiles * 4).unwrap();

            let mut ib_data = Vec::with_capacity(nb_tiles * 6);

            let mut rng = rand::thread_rng();
            let pick = Range::new(0f64, 1.);

            for (num, sprite) in vb.map().chunks_mut(4).enumerate() {

                let value = pick.ind_sample(&mut rng);
                let absolute_tex_id: u32 = if value  < 0.25 {
                    tex_id * TILES_PER_TEXTURE + 6
                } else if value < 0.75 {
                    tex_id * TILES_PER_TEXTURE + 7
                } else {
                    tex_id * TILES_PER_TEXTURE + 8
                };

                let num = num as u32;
                let half_tile = GAME_UNIT_TO_PX;
                let x = (num % width) as i64 - (width / 2) as i64;
                let y = (num / width) as i64 - (height / 2) as i64;
                let position: (f32, f32) = (x as f32 * 16., y as f32 * 16.);

                sprite[0].i_position[0] = position.0 - half_tile;
                sprite[0].i_position[1] = position.1 + half_tile;
                sprite[0].i_tex_id = absolute_tex_id;
                sprite[1].i_position[0] = position.0 + half_tile;
                sprite[1].i_position[1] = position.1 + half_tile;
                sprite[1].i_tex_id = absolute_tex_id;
                sprite[2].i_position[0] = position.0 - half_tile;
                sprite[2].i_position[1] = position.1 - half_tile;
                sprite[2].i_tex_id = absolute_tex_id;
                sprite[3].i_position[0] = position.0 + half_tile;
                sprite[3].i_position[1] = position.1 - half_tile;
                sprite[3].i_tex_id = absolute_tex_id;

                ib_data.push(num * 4);
                ib_data.push(num * 4 + 1);
                ib_data.push(num * 4 + 2);
                ib_data.push(num * 4 + 1);
                ib_data.push(num * 4 + 3);
                ib_data.push(num * 4 + 2);
            }

            (vb, IndexBuffer::new(display, PrimitiveType::TrianglesList, &ib_data).unwrap())
        };

        self.vertex_buffer = vertex_buffer;
        self.index_buffer = index_buffer;
    }

    pub fn render(&self, target: &mut Frame, mvp: &Matrix4<f32>, draw_parameters: &DrawParameters) {

        use glium::uniforms::MagnifySamplerFilter::Nearest;
        use glium::uniforms::MinifySamplerFilter::NearestMipmapNearest;

        let uniforms = uniform! {
            mvp: *mvp,
            tex: self.texture.sampled()
                .minify_filter(NearestMipmapNearest)
                .magnify_filter(Nearest)
        };

        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &draw_parameters
        ).unwrap();
    }
}
