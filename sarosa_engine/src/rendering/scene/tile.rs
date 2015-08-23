
use glium::VertexBuffer;
use rendering::renderer::shaders;
use animation::TextureId;
use Window;

impl TileLayer {

    pub fn new(window: &Window, width: u32, height: u32) -> TileLayer {

        let display = window.display;

        TileLayer {
            tex_ids: vec![UNINITIALIZED_TEXTURE_ID; width * height],
            vertex_buffer: VertexBuffer::empty(display, 0).unwrap(),
            index_buffer: IndexBuffer::empty(display, PrimitiveType::TrianglesList, 0).unwrap(),
            width: width,
        }
    }

    pub fn set_tex_id(&mut self, x: u32, y:u32, tex_id: TextureId) {
        self.tex_ids[x * y] = tex_id;
    }

    pub fn update_on_camera_change(&mut self, camera: &Camera) {
        let (vertex_buffer, index_buffer) = {

            let mut vb: VertexBuffer<shaders::map::Vertex> =
                VertexBuffer::empty(display, nb_tiles * 4).unwrap();

            let mut ib_data = Vec::with_capacity(nb_tiles * 6);
        };


    }
}
