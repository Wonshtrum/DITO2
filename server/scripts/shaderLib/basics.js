'use strict';


//=================================================================================================
// INCLUDES

const I_OFFSETS_STRIP = `
    const vec2 offsets[4] = vec2[](vec2(0, 0), vec2(1, 0), vec2(0, 1), vec2(1, 1));
    #define a_offset offsets[gl_VertexID]
`
const I_OFFSETS = `
    const vec2 offsets[6] = vec2[](vec2(0, 0), vec2(1, 0), vec2(0, 1), vec2(1, 0), vec2(0, 1), vec2(1, 1));
    #define a_offset offsets[gl_VertexID]
`
const I_CAMERA = `
    uniform vec2 u_screen;
    uniform vec3 u_camera;

    vec4 projection(vec2 p) {
        return vec4((p - u_camera.xy) * u_camera.zz / u_screen, 0.0, 1.0);
    }
`
const I_ATLAS = `
    const float block_size = 8.;
    const vec2 atlas_size = vec2(4, 14);
    const ivec2 atlas[14] = ivec2[](
        ivec2(1),
        ivec2(1),
        ivec2(1),
        ivec2(1),
        ivec2(1),
        ivec2(10, 2),
        ivec2(15, 3),
        ivec2(1),
        ivec2(1),
        ivec2(10, 2),
        ivec2(1),
        ivec2(1),
        ivec2(1),
        ivec2(1)
    );
`

//=================================================================================================
// VERTEX SHADERS

const basic_vsh = compileShader(gl.VERTEX_SHADER, `
    layout(location = 0) in vec2 a_position;
    layout(location = 1) in vec4 a_color;
    out vec4 v_color;

    void main() {
        v_color = a_color;
        gl_Position = projection(a_position);
    }
`, I_CAMERA);

const batch_vsh = compileShader(gl.VERTEX_SHADER, `
    layout(location = 0) in vec2 a_position;
    layout(location = 1) in vec2 a_size;
    layout(location = 2) in float a_tex;
    layout(location = 3) in vec4 a_color;
    flat out float v_tex;
    out vec2 v_uv;
    out vec4 v_color;

    void main() {
        vec2 position = a_position + a_offset * a_size;
        v_tex = a_tex;
        v_uv = a_offset;
        v_color = a_color;
        gl_Position = projection(position);
    }
`, I_OFFSETS, I_CAMERA);

const particule_vsh = compileShader(gl.VERTEX_SHADER, `
    layout(location = 0) in vec2 a_t;
    layout(location = 1) in vec3 a_x;
    layout(location = 2) in vec3 a_y;
    layout(location = 3) in vec3 a_s;
    layout(location = 4) in vec4 a_c1;
    layout(location = 5) in vec4 a_c2;
    out vec4 v_color;
    uniform float u_t;

    void main() {
        float t = (u_t - a_t.x)/(a_t.y - a_t.x);
        float x = a_x.x * t*t + a_x.y * t + a_x.z;
        float y = a_y.x * t*t + a_y.y * t + a_y.z;
        float s = a_s.x * t*t + a_s.y * t + a_s.z;
        vec2 position = vec2(x, y) + a_offset * s;
        v_color = mix(a_c1, a_c2, t);
        gl_Position = projection(position);
    }
`, I_OFFSETS, I_CAMERA);

//=================================================================================================
// FRAGMENT SHADERS

const clear_fsv = compileShader(gl.FRAGMENT_SHADER, `
    layout(location = 0) out vec4 outColor;
    uniform vec4 u_color;

    void main() {
        outColor = u_color;
    }
`);

const basic_fsv = compileShader(gl.FRAGMENT_SHADER, `
    layout(location = 0) out vec4 outColor;
    in vec4 v_color;

    void main() {
        outColor = v_color;
    }
`);

const texture_fsv = compileShader(gl.FRAGMENT_SHADER, `
    layout(location = 0) out vec4 outColor;
    flat in float v_tex;
    in vec2 v_uv;
    in vec4 v_color;
    uniform sampler2D u_tex;
    uniform int u_t;
    uniform bool u_debug;

    void main() {
        if (u_debug) {
            outColor = vec4(0, 0, 0, 1);
            return;
        }
        ivec2 anim = atlas[int(v_tex)];
        int anim_id = (u_t / anim.x) % anim.y;
        vec2 coords = vec2(anim_id, v_tex + 1.) + v_uv * vec2(1, -1);
        outColor = texture(u_tex, coords / atlas_size) * v_color;
    }
`, I_ATLAS);

const SHADERS = {};

SHADERS.basic = new Shader(basic_vsh, basic_fsv);
SHADERS.clear = new Shader(basic_vsh, clear_fsv);
SHADERS.batch = new Shader(batch_vsh, texture_fsv);
SHADERS.particule = new Shader(particule_vsh, basic_fsv);
