#[derive(Copy, Clone)]
pub struct Vertex {
    pub i_position: [f32; 2],
    pub i_tex_id: u32,
}

implement_vertex!(Vertex, i_position, i_tex_id);

pub const VERTEX_140: &'static str = r"
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
";

pub const FRAGMENT_140: &'static str = r"
    #version 140
    uniform sampler2DArray tex;
    in vec2 v_tex_coords;
    flat in uint v_tex_id;
    out vec4 f_color;
    void main() {
        f_color = texture(tex, vec3(v_tex_coords, float(v_tex_id)));
    }
";

pub const VERTEX_110: &'static str = r"
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
";

pub const FRAGMENT_110: &'static str = r"
    #version 110
    uniform sampler2DArray tex;
    varying vec2 v_tex_coords;
    flat varying uint v_tex_id;
    void main() {
        gl_FragColor = texture2DArray(tex, vec3(v_tex_coords, float(v_tex_id)));
    }
";

pub const VERTEX_100: &'static str = r"
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
";

pub const FRAGMENT_100: &'static str = r"
    #version 100
    uniform sampler2DArray tex;
    varying lowp vec2 v_tex_coords;
    flat varying uint v_tex_id;
    void main() {
        gl_FragColor = texture2DArray(tex, vec3(v_tex_coords, float(v_tex_id)));
    }
";