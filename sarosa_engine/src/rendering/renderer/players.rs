use std::cmp;
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
use glium::VertexBuffer;
use glium::Frame;

use models::game::GameData;
use animation::AbsoluteTextureId;
use rendering::renderer::shaders;
use Window;

pub struct PlayersRenderer {
    program: Program,
    vertex_buffer: VertexBuffer<shaders::players::Vertex>,
    texture: Texture2dArray,
    index_buffer: IndexBuffer<u16>,
    nb_sprites: usize,
}

const MAX_SPRITES: usize = 1024;

impl PlayersRenderer {

    pub fn new(window: &Window) -> PlayersRenderer {

        let ref display = window.display;

        let texture = {
            // TODO(Nemikolh): Use a ResourceManager to load
            // them before being here and do something clever with it.
            let images = vec![
                image::open("./assets/players/Kiwan.png").unwrap(),
                image::open("./assets/players/Vurf.png").unwrap()
            ];

            Texture2dArray::new(display, images).unwrap()
        };

        let (vertex_buffer, index_buffer) = {
            let vb: VertexBuffer<shaders::players::Vertex> =
                VertexBuffer::empty_dynamic(display, MAX_SPRITES * 4).unwrap();

            let mut ib_data = Vec::with_capacity(MAX_SPRITES * 6);

            for num in 0..MAX_SPRITES {
                let num = num as u16;
                ib_data.push(num * 4);
                ib_data.push(num * 4 + 1);
                ib_data.push(num * 4 + 2);
                ib_data.push(num * 4 + 1);
                ib_data.push(num * 4 + 3);
                ib_data.push(num * 4 + 2);
            }
            (vb, IndexBuffer::new(display, PrimitiveType::TrianglesList, &ib_data).unwrap())
        };

        // TODO(Nemikolh): Add support back for different shader version.
        let program = program!(display,
            140 => {
                vertex: shaders::players::VERTEX_140,
                fragment: shaders::players::FRAGMENT_140
            },
            //
            // 110 => {
            //     vertex: shaders::players::VERTEX_110,
            //     fragment: shaders::players::FRAGMENT_110
            // },
            //
            // 100 => {
            //     vertex: shaders::players::VERTEX_100,
            //     fragment: shaders::players::FRAGMENT_100
            // },
        ).unwrap();

        PlayersRenderer {
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture: texture,
            nb_sprites: 0,
        }
    }

    pub fn update_gpu_mem(&mut self, game_data: &GameData) {

        self.nb_sprites = game_data.players_len();

        let mut mapping = self.vertex_buffer.map();
        for (sprite, player) in mapping.chunks_mut(4).zip(game_data.iter_players()) {

            let AbsoluteTextureId(tex_id) = player.animator.absolute_tex_id();
            let (x, y) = (player.position.x, player.position.y);

            sprite[0].i_position[0] = x - 12.0;
            sprite[0].i_position[1] = y + 16.0;
            sprite[1].i_position[0] = x + 12.0;
            sprite[1].i_position[1] = y + 16.0;
            sprite[2].i_position[0] = x - 12.0;
            sprite[2].i_position[1] = y - 16.0;
            sprite[3].i_position[0] = x + 12.0;
            sprite[3].i_position[1] = y - 16.0;
            sprite[0].i_tex_id = tex_id;
            sprite[1].i_tex_id = tex_id;
            sprite[2].i_tex_id = tex_id;
            sprite[3].i_tex_id = tex_id;
        }
    }

    pub fn render(&self, target: &mut Frame, mvp: &Matrix4<f32>, draw_parameters: &DrawParameters) {

        use glium::uniforms::MagnifySamplerFilter::Nearest;
        use glium::uniforms::MinifySamplerFilter::NearestMipmapNearest;

        if self.nb_sprites == 0 {
            return;
        }

        // we must only draw the number of sprites that we have written in the vertex buffer
        // if you only want to draw 20 sprites for example, you should pass `0 .. 20 * 6` instead
        let sprites = cmp::min(self.nb_sprites, MAX_SPRITES);
        let ib_slice = self.index_buffer.slice(0 .. sprites * 6).unwrap();

        let uniforms = uniform! {
            mvp: *mvp,
            tex: self.texture.sampled()
                .minify_filter(NearestMipmapNearest)
                .magnify_filter(Nearest)
        };

        target.draw(
            &self.vertex_buffer,
            &ib_slice,
            &self.program,
            &uniforms,
            draw_parameters
        ).unwrap();

    }
}
