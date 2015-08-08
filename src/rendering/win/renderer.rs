extern crate rand;

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
use rendering::scene::WorldScene;

pub struct GameRenderer {
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    texture: Texture2dArray,
}

#[derive(Copy, Clone)]
struct Vertex {
    i_position: [f32; 2],
    i_tex_id: u32,
}

const SPRITES_COUNT: usize = 1024;

implement_vertex!(Vertex, i_position, i_tex_id);

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
        let (mut vertex_buffer, index_buffer) = {

            let mut vb: VertexBuffer<Vertex> = VertexBuffer::empty_dynamic(display,
                                                                            SPRITES_COUNT * 4).unwrap();
            let mut ib_data = Vec::with_capacity(SPRITES_COUNT * 6);

            // initializing with random data
            for (num, sprite) in vb.map().chunks_mut(4).enumerate() {
                let tex_id: u32 = rand::random();
                let tex_id = tex_id % texture.get_array_size().unwrap();
                let position: (f32, f32) = (rand::random(), rand::random());
                let position: (f32, f32) = (position.0 * 2.0 - 1.0, position.1 * 2.0 - 1.0);

                sprite[0].i_position[0] = position.0 - 0.1;
                sprite[0].i_position[1] = position.1 + 0.1;
                sprite[0].i_tex_id = tex_id;
                sprite[1].i_position[0] = position.0 + 0.1;
                sprite[1].i_position[1] = position.1 + 0.1;
                sprite[1].i_tex_id = tex_id;
                sprite[2].i_position[0] = position.0 - 0.1;
                sprite[2].i_position[1] = position.1 - 0.1;
                sprite[2].i_tex_id = tex_id;
                sprite[3].i_position[0] = position.0 + 0.1;
                sprite[3].i_position[1] = position.1 - 0.1;
                sprite[3].i_tex_id = tex_id;

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
                vertex: "
                    #version 140
                    in vec2 i_position;
                    in uint i_tex_id;
                    out vec2 v_tex_coords;
                    flat out uint v_tex_id;
                    void main() {
                        gl_Position = vec4(i_position, 0.0, 1.0);
                        if (gl_VertexID % 4 == 0) {
                            v_tex_coords = vec2(0.0, 1.0);
                        } else if (gl_VertexID % 4 == 1) {
                            v_tex_coords = vec2(1.0, 1.0);
                        } else if (gl_VertexID % 4 == 2) {
                            v_tex_coords = vec2(0.0, 0.0);
                        } else {
                            v_tex_coords = vec2(1.0, 0.0);
                        }
                        v_tex_id = i_tex_id;
                    }
                ",

                fragment: "
                    #version 140
                    uniform sampler2DArray tex;
                    in vec2 v_tex_coords;
                    flat in uint v_tex_id;
                    out vec4 f_color;
                    void main() {
                        f_color = texture(tex, vec3(v_tex_coords, float(v_tex_id)));
                    }
                "
            },

            110 => {
                vertex: "
                    #version 110
                    in vec2 i_position;
                    in uint i_tex_id;
                    varying vec2 v_tex_coords;
                    flat varying uint v_tex_id;
                    void main() {
                        gl_Position = vec4(i_position, 0.0, 1.0);
                        if (gl_VertexID % 4 == 0) {
                            v_tex_coords = vec2(0.0, 1.0);
                        } else if (gl_VertexID % 4 == 1) {
                            v_tex_coords = vec2(1.0, 1.0);
                        } else if (gl_VertexID % 4 == 2) {
                            v_tex_coords = vec2(0.0, 0.0);
                        } else {
                            v_tex_coords = vec2(1.0, 0.0);
                        }
                        v_tex_id = i_tex_id;
                    }
                ",

                fragment: "
                    #version 110
                    uniform sampler2DArray tex;
                    varying vec2 v_tex_coords;
                    flat varying uint v_tex_id;
                    void main() {
                        gl_FragColor = texture2DArray(tex, vec3(v_tex_coords, float(v_tex_id)));
                    }
                "
            },

            100 => {
                vertex: "
                    #version 100
                    attribute lowp vec2 i_position;
                    attribute uint i_tex_id;
                    varying lowp vec2 v_tex_coords;
                    flat varying uint v_tex_id;
                    void main() {
                        gl_Position = vec4(i_position, 0.0, 1.0);
                        if (gl_VertexID % 4 == 0) {
                            v_tex_coords = vec2(0.0, 1.0);
                        } else if (gl_VertexID % 4 == 1) {
                            v_tex_coords = vec2(1.0, 1.0);
                        } else if (gl_VertexID % 4 == 2) {
                            v_tex_coords = vec2(0.0, 0.0);
                        } else {
                            v_tex_coords = vec2(1.0, 0.0);
                        }
                        v_tex_id = i_tex_id;
                    }
                ",

                fragment: "
                    #version 100
                    uniform sampler2DArray tex;
                    varying lowp vec2 v_tex_coords;
                    flat varying uint v_tex_id;
                    void main() {
                        gl_FragColor = texture2DArray(tex, vec3(v_tex_coords, float(v_tex_id)));
                    }
                "
            },
        ).unwrap();

        GameRenderer {
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture: texture,
        }
    }

    pub fn do_random_stuff(&mut self) {
        // moving the sprites in a random direction
        // in a game, you would typically write the exact positions and texture IDs of your sprites
        let mut mapping = self.vertex_buffer.map();
        for sprite in mapping.chunks_mut(4) {
            let mv: (f32, f32) = (rand::random(), rand::random());
            let mv = (mv.0 * 0.01 - 0.005, mv.1 * 0.01 - 0.005);

            sprite[0].i_position[0] += mv.0;
            sprite[0].i_position[1] += mv.1;
            sprite[1].i_position[0] += mv.0;
            sprite[1].i_position[1] += mv.1;
            sprite[2].i_position[0] += mv.0;
            sprite[2].i_position[1] += mv.1;
            sprite[3].i_position[0] += mv.0;
            sprite[3].i_position[1] += mv.1;
            // sprite[...].i_tex_id = ...;  // if you want to set the texture
        }
    }

    pub fn render(&self, world_scene: &WorldScene, window: &mut Window) {

        // we must only draw the number of sprites that we have written in the vertex buffer
        // if you only want to draw 20 sprites for example, you should pass `0 .. 20 * 6` instead
        let ib_slice = self.index_buffer.slice(0 .. SPRITES_COUNT * 6).unwrap();

        // drawing a frame
        let mut target = window.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&self.vertex_buffer, &ib_slice,
                    &self.program, &uniform! { tex: &self.texture }, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
