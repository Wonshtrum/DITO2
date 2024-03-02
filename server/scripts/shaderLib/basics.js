'use strict';


//=================================================================================================
// INCLUDES

const A_OFFSETS_STRIP = `
    vec2 offsets[4] = vec2[](vec2(0, 0), vec2(1, 0), vec2(0, 1), vec2(1, 1));
    #define a_offset offsets[gl_VertexID]
`
const A_OFFSETS = `
    vec2 offsets[6] = vec2[](vec2(0, 0), vec2(1, 0), vec2(0, 1), vec2(1, 0), vec2(0, 1), vec2(1, 1));
    #define a_offset offsets[gl_VertexID]
`
const CAMERA = `
    uniform vec2 u_screen;
    uniform vec3 u_camera;

    vec4 projection(vec2 p) {
        return vec4((p - u_camera.xy) * u_camera.zz / u_screen, 0.0, 1.0);
    }
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
`, CAMERA);

const batch_vsh = compileShader(gl.VERTEX_SHADER, `
    layout(location = 0) in vec2 a_position;
    layout(location = 1) in vec2 a_size;
    layout(location = 2) in vec4 a_color;
    out vec4 v_color;

    void main() {
        vec2 position = a_position + a_offset * a_size;
        v_color = a_color;
        gl_Position = projection(position);
    }
`, A_OFFSETS, CAMERA);

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


const SHADERS = {};

SHADERS.basic = new Shader(basic_vsh, basic_fsv);
SHADERS.batch = new Shader(batch_vsh, basic_fsv);
SHADERS.clear = new Shader(basic_vsh, clear_fsv);
