extern crate rand;

use std::cmp;
use glium::{Surface};
use glium::index::{
    PrimitiveType,
    IndexBuffer
};
use glium::texture::Texture2dArray;
use glium::program::Program;
use glium::{
    VertexBuffer
};
use Window;
use models::game::GameData;
use rendering::scene::WorldScene;

mod shaders;

pub struct GameRenderer {
    program: Program,
    vertex_buffer: VertexBuffer<shaders::Vertex>,
    index_buffer: IndexBuffer<u16>,
    texture: Texture2dArray,
    nb_sprites: usize,
}

const MAX_SPRITES: usize = 1024;

impl GameRenderer {

    pub fn new(window: &Window) -> GameRenderer {

        let ref display = window.display;

        // generating a bunch of unicolor 2D images that will be used for a texture
        // we store all of them in a `Texture2dArray`
        let texture = {
            let images = (0 .. 64).map(|_| {
                let color1: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
                let color2: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
                vec![vec![color1], vec![color2]]
            }).collect::<Vec<_>>();

            Texture2dArray::new(display, images).unwrap()
        };

        // building the vertex buffer and index buffers that will be filled with the data of
        // the sprites
        let (vertex_buffer, index_buffer) = {
            let vb: VertexBuffer<shaders::Vertex> = VertexBuffer::empty_dynamic(display,
                                                                            MAX_SPRITES * 4).unwrap();
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

        // we determine the texture coordinates depending on the ID the of vertex
        let program = program!(display,
            140 => {
                vertex: shaders::VERTEX_140,
                fragment: shaders::FRAGMENT_140
            },

            110 => {
                vertex: shaders::VERTEX_110,
                fragment: shaders::FRAGMENT_110
            },

            100 => {
                vertex: shaders::VERTEX_100,
                fragment: shaders::FRAGMENT_100
            },
        ).unwrap();

        GameRenderer {
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture: texture,
            nb_sprites: 0,
        }
    }

    pub fn initialize_gpu_mem(&mut self, game_data: &GameData) {

        self.nb_sprites = game_data.players_len();

        // initializing with random data
        for (sprite, player) in self.vertex_buffer.map().chunks_mut(4).zip(game_data.iter_players()) {
            let tex_id: u32 = rand::random();
            let tex_id = tex_id % self.texture.get_array_size().unwrap();
            let (x, y) = (player.position.x, player.position.y);
            // let position: (f32, f32) = (rand::random(), rand::random());
            // let position: (f32, f32) = (position.0 * 2.0 - 1.0, position.1 * 2.0 - 1.0);

            sprite[0].i_position[0] = x - 0.1;
            sprite[0].i_position[1] = y + 0.1;
            sprite[1].i_position[0] = x + 0.1;
            sprite[1].i_position[1] = y + 0.1;
            sprite[2].i_position[0] = x - 0.1;
            sprite[2].i_position[1] = y - 0.1;
            sprite[3].i_position[0] = x + 0.1;
            sprite[3].i_position[1] = y - 0.1;
            sprite[0].i_tex_id = tex_id;
            sprite[2].i_tex_id = tex_id;
            sprite[1].i_tex_id = tex_id;
            sprite[3].i_tex_id = tex_id;
        }
    }

    pub fn update_gpu_mem(&mut self, game_data: &GameData) {
        // This function can only be called if this assertion is true.
        assert_eq!(game_data.players_len(), self.nb_sprites);

        // moving the sprites in a random direction
        // in a game, you would typically write the exact positions and texture IDs of your sprites
        let mut mapping = self.vertex_buffer.map();
        for (sprite, player) in mapping.chunks_mut(4).zip(game_data.iter_players()) {
            // let mv: (f32, f32) = (rand::random(), rand::random());
            // let mv = (mv.0 * 0.01 - 0.005, mv.1 * 0.01 - 0.005);
            let (x, y) = (player.position.x, player.position.y);

            sprite[0].i_position[0] = x - 0.1;
            sprite[0].i_position[1] = y + 0.1;
            sprite[1].i_position[0] = x + 0.1;
            sprite[1].i_position[1] = y + 0.1;
            sprite[2].i_position[0] = x - 0.1;
            sprite[2].i_position[1] = y - 0.1;
            sprite[3].i_position[0] = x + 0.1;
            sprite[3].i_position[1] = y - 0.1;
            // sprite[...].i_tex_id = ...;  // if you want to set the texture
        }
    }

    pub fn render(&self, world_scene: &WorldScene, window: &mut Window) {

        if self.nb_sprites == 0 {
            return;
        }

        // we must only draw the number of sprites that we have written in the vertex buffer
        // if you only want to draw 20 sprites for example, you should pass `0 .. 20 * 6` instead
        let sprites = cmp::min(self.nb_sprites, MAX_SPRITES);
        let ib_slice = self.index_buffer.slice(0 .. sprites * 6).unwrap();

        // drawing a frame
        let mut target = window.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&self.vertex_buffer, &ib_slice,
                    &self.program, &uniform! { tex: &self.texture }, &Default::default()).unwrap();

        // Render scene:
        world_scene.render(&target);
        target.finish().unwrap();
    }
}
