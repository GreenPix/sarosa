
use glium::texture::Texture2dArray;
use glium::VertexBuffer;


pub struct PlayersRenderer {
    vertex_buffer: VertexBuffer<shaders::Vertex>,
    texture: Texture2dArray,
}
