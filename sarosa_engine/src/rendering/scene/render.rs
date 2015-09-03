use rendering::scene::Map;
use rendering::scene::TileLayerWithDepth;
use rendering::scene::TileLayer;
use rendering::scene::ObjectLayer;
use rendering::scene::Object;
use rendering::scene::Vertex;

use cgmath::Matrix4;
use glium::program::Program;
use glium::Frame;
use glium::DrawParameters;
use glium::VertexBuffer;
use glium::index::IndexBufferSlice;
use glium::index::IndexBuffer;
use glium::texture::Texture2dArray;
use glium::draw_parameters::DepthTest;

impl WorldScene {

    pub fn render(&self, target: &mut Frame, program: &Program) {

        self.map.render(target, program, self.camera.as_uniform());
    }
}

impl Map {

    fn render(&self, target: &mut Frame, program: &Program, mvp: &Matrix4<f32>) {

        for layer in self.layers.iter() {
            let nb_tiles = self.viewport.width * self.viewport.height;
            layer.render(
                target,
                program,
                mvp,
                &self.vertices,
                &self.indices.slice(0 .. nb_tiles * 6).unwrap(),
                &self.chipsets_texture
            );
        }

        self.objects.render(
            target,
            program,
            mvp
        );
    }
}

impl TileLayerWithDepth {

    fn render<'a>(&self,
        target: &mut Frame,
        program: &Program,
        mvp: &Matrix4<f32>,
        vertices: &VertexBuffer<Vertex>,
        indices: &IndexBufferSlice<'a, u32>,
        texture: &Texture2dArray)
    {
        self.0.render(
            target,
            program,
            mvp,
            vertices,
            indices,
            texture
        );
    }
}

impl TileLayer {

    fn render<'a>(&self,
        target: &mut Frame,
        program: &Program,
        mvp: &Matrix4<f32>,
        vertices: &VertexBuffer<Vertex>,
        indices: &IndexBufferSlice<'a, u32>,
        texture: &Texture2dArray)
    {
        use glium::uniforms::MagnifySamplerFilter::Nearest;
        use glium::uniforms::MinifySamplerFilter::NearestMipmapNearest;

        let uniforms = uniform! {
            mvp: *mvp,
            tex: texture.sampled()
                .minify_filter(NearestMipmapNearest)
                .magnify_filter(Nearest)
        };

        let draw_parameters = DrawParameters {
            blending_function: Some(
                Addition { source: SourceAlpha, destination: OneMinusSourceAlpha }
            ),
            .. Default::default()
        };

        target.draw(
            (vertices, &self.current_tex_buffer),
            indices,
            program,
            &uniforms,
            &draw_parameters
        );
    }
}

impl ObjectLayer {

    fn render(&self,
        target: &mut Frame,
        program: &Program,
        mvp: &Matrix4<f32>)
    {
        let draw_parameters = DrawParameters {
            blending_function: Some(
                Addition { source: SourceAlpha, destination: OneMinusSourceAlpha }
            ),
            depth_test: DepthTest::IfLess,
            depth_write: true,
            .. Default::default()
        };

        for object in self.objects.iter() {
            object.render(target, program, mvp, &draw_parameters);
        }
    }
}

impl Object {

    fn render(&self,
        target: &mut Frame,
        program: &Program,
        mvp: &Matrix4<f32>,
        draw_parameters: &DrawParameters)
    {
        use glium::uniforms::MagnifySamplerFilter::Nearest;
        use glium::uniforms::MinifySamplerFilter::NearestMipmapNearest;

        let uniforms = uniform! {
            mvp: *mvp,
            tex: self.frames_texture.sampled()
                .minify_filter(NearestMipmapNearest)
                .magnify_filter(Nearest)
        };

        target.draw(
            (&self.vertices, self.inst_attr.per_instance().unwrap()),
            &self.indices,
            program,
            &uniforms,
            draw_parameters
        );
    }
}
