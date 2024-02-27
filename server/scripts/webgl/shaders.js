'use strict';


class Shader {
    constructor(vertShader, fragShader) {
        this.program = gl.createProgram();
        gl.attachShader(this.program, vertShader);
        gl.attachShader(this.program, fragShader);
        gl.linkProgram(this.program);
        if (!gl.getProgramParameter(this.program, gl.LINK_STATUS)) {
            console.error(gl.getProgramInfoLog(this.program));
        }
        //UNIFORMS
        this.uniforms = getUniforms(this.program);
    }
    bind() {
        gl.useProgram(this.program);
    }
    unbind() {
        gl.useProgram(0);
    }
};

function preprocess(type, source, includes) {
    return `#version 300 es
precision mediump float;
#define SHADER_TYPE ${type === gl.VERTEX_SHADER ? 0 : 1}

//=====================================\n`
        + (includes.is_empty() ? source : source.replace(/(\s*void\s*main\s*\()/,
            "\n\n//=====================================\n"
            + includes.join("\n//-------------------------------------\n")
            + "\n//=====================================$1"
        ));
};

function compileShader(type, source, ...includes) {
    source = preprocess(type, source, includes);
    let shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error(type === gl.VERTEX_SHADER ? "Vert" : "Frag", gl.getShaderInfoLog(shader));
        console.error(source);
    }
    return shader
};

function getUniforms(program) {
    let uniforms = {};
    let uniformCount = gl.getProgramParameter(program, gl.ACTIVE_UNIFORMS);
    for (let i = 0; i < uniformCount; i++) {
        let uniformName = gl.getActiveUniform(program, i).name;
        uniforms[uniformName] = gl.getUniformLocation(program, uniformName);
    }
    return uniforms;
};
