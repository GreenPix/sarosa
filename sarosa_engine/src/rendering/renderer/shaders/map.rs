#[derive(Copy, Clone)]
pub struct Vertex {
    pub i_position: [f32; 2],
    pub i_tex_id: u32,
}

implement_vertex!(Vertex, i_position, i_tex_id);

pub const VERTEX_140: &'static str = r"
    #version 140
    uniform mat4 mvp;
    in vec2 i_position;
    in uint i_tex_id;
    out vec2 v_tex_coords;
    flat out uint v_tex_id;
    void main() {
        gl_Position = mvp * vec4(i_position, 0.0, 1.0);
        uint sprite_x = i_tex_id % uint(30 * 16);
        uint sprite_y = sprite_x / uint(30);
        sprite_x = sprite_x % uint(30);
        if (gl_VertexID % 4 == 0) {
            v_tex_coords = vec2(float(sprite_x) * 1.0 / 30,     float(sprite_y) * 1.0 / 16);
        } else if (gl_VertexID % 4 == 1) {
            v_tex_coords = vec2(float(sprite_x + uint(1)) * 1.0 / 30, float(sprite_y) * 1.0 / 16);
        } else if (gl_VertexID % 4 == 2) {
            v_tex_coords = vec2(float(sprite_x) * 1.0 / 30,     float(sprite_y + uint(1)) * 1.0 / 16);
        } else {
            v_tex_coords = vec2(float(sprite_x + uint(1)) * 1.0 / 30, float(sprite_y + uint(1)) * 1.0 / 16);
        }
        v_tex_id = (i_tex_id / uint(30 * 16)) + 1;
    }
";// If the input for i_tex_id is zero then we should do
// gl_Position = vec4(0.0);
// v_tex_id = 0;
// v_tex_coords = vec2(0.0);

pub const FRAGMENT_140: &'static str = r"
    #version 140
    uniform sampler2DArray tex;
    in vec2 v_tex_coords;
    flat in uint v_tex_id;
    out vec4 f_color;
    void main() {
        if (v_tex_id == 0) {
            discard();
        } else {
            vec3 gamma = vec3(2.2);
            vec4 tex_color = texture(tex, vec3(v_tex_coords.x, 1.0 - v_tex_coords.y, float(v_tex_id - 1)));
            f_color = vec4(pow(tex_color.rgb, gamma), tex_color.a);
        }
    }
";
