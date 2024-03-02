'use strict';


//=================================================================================================

class Texture {
    constructor(width, height, parameters, id = 0, pixels = null) {
        this.width = width;
        this.height = height;
        this.data = gl.createTexture();
        gl.activeTexture(gl.TEXTURE0 + id);
        gl.bindTexture(gl.TEXTURE_2D, this.data);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, parameters.minFilter);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, parameters.maxFilter);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
        gl.texImage2D(gl.TEXTURE_2D, 0, parameters.internalFormat, width, height, 0, parameters.format, parameters.type, pixels);
        gl.generateMipmap(gl.TEXTURE_2D);
    }

    attach(id) {
        gl.activeTexture(gl.TEXTURE0 + id);
        gl.bindTexture(gl.TEXTURE_2D, this.data);
        return id;
    }
}

class AttachmentParameters {
    constructor(name, maxFilter, minFilter, internalFormat, format, type) {
        this.name = name;
        this.maxFilter = maxFilter;
        this.minFilter = minFilter;
        this.internalFormat = internalFormat;
        this.format = format;
        this.type = type;
    }
};

function AP(name = "main", maxFilter = gl.NEAREST, minFilter = gl.NEAREST, internalFormat = gl.RGBA8, format = gl.RGBA, type = gl.UNSIGNED_BYTE) {
    return new AttachmentParameters(name, maxFilter, minFilter, internalFormat, format, type);
}

function loadTexture(url, parameters = new AP()) {
    let img = new Image();
    let texture = new Texture(0, 0, parameters);
    img.onload = function () {
        console.log(url, "loaded");
        texture.width = img.width;
        texture.height = img.height;
        texture.attach(0);
        gl.texImage2D(gl.TEXTURE_2D, 0, parameters.internalFormat, img.width, img.height, 0, parameters.format, parameters.type, img);
    };
    img.src = "img/" + url;
    return texture;
}

//=================================================================================================

class FBO {
    constructor(width, height, attachments) {
        this.n = attachments.length;
        this.textures = {};
        this.attachments = [];

        this.width = width;
        this.height = height;

        this.texelSizeX = 1.0 / width;
        this.texelSizeY = 1.0 / height;

        this.defaultViewPort();

        this.fbo = gl.createFramebuffer();
        gl.bindFramebuffer(gl.FRAMEBUFFER, this.fbo);

        for (let i = 0; i < this.n; i++) {
            this.attachments.push(gl.COLOR_ATTACHMENT0 + i);
            let texture = new Texture(width, height, attachments[i], i);
            gl.framebufferTexture2D(gl.FRAMEBUFFER, this.attachments[i], gl.TEXTURE_2D, texture.data, 0);
            this.textures[attachments[i].name] = texture;
            if (i === 0) {
                this.texture = texture;
            }
        }
        if (gl.checkFramebufferStatus(gl.FRAMEBUFFER) !== gl.FRAMEBUFFER_COMPLETE) {
            console.error("FRAMEBUFFER NOT READY");
        }

        this.bind();
    }

    defaultViewPort() {
        this.vpx = 0;
        this.vpy = 0;
        this.vpw = this.width;
        this.vph = this.height;
    }

    setViewPort(x, y, width, height) {
        this.vpx = x;
        this.vpy = y;
        this.vpw = width;
        this.vph = height;
    }
    centerViewPort(x, y, halfWidth, halfHeight) {
        this.vpx = x - halfWidth;
        this.vpy = y - halfHeight;
        this.vpw = 2 * halfWidth;
        this.vph = 2 * halfHeight;
    }

    bind() {
        gl.viewport(this.vpx, this.vpy, this.vpw, this.vph);
        gl.bindFramebuffer(gl.FRAMEBUFFER, this.fbo);
        gl.drawBuffers(this.attachments);
    }
};

class RWFBO {
    constructor(width, height, attachments) {
        this.width = width;
        this.height = height;

        this.texelSizeX = 1.0 / width;
        this.texelSizeY = 1.0 / height;

        this.read = new FBO(width, height, attachments);
        this.write = new FBO(width, height, attachments);
    }

    swap() {
        [this.read, this.write] = [this.write, this.read];
    }
};

function unbindAllFBO() {
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.drawBuffers([gl.BACK]);
    gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
}

//=================================================================================================

function apply_layout(layout, divisor = 1) {
    let vertexSize = layout.sum();
    let stride = 0;
    for (let i = 0; i < layout.length; i++) {
        gl.enableVertexAttribArray(i);
        gl.vertexAttribPointer(i, layout[i], gl.FLOAT, false, vertexSize * FLOAT_SIZE, stride * FLOAT_SIZE);
        gl.vertexAttribDivisor(i, divisor);
        stride += layout[i];
    }
}

//=================================================================================================

const RenderPass = {
    va: gl.createVertexArray(),
    pass() {
        gl.bindVertexArray(this.va)
        gl.drawArrays(gl.TRIANGLE_STRIP, 0, 3);
    }
};

function blit(target, clear = false) {
    if (target === null) {
        unbindAllFBO();
    } else {
        target.bind();
    }
    if (clear) {
        gl.clearColor(0.0, 0.0, 0.0, 1.0);
        gl.clear(gl.COLOR_BUFFER_BIT);
    }
    RenderPass.pass();
};

const transferTarget = (() => {
    const transferVertexShader = compileShader(gl.VERTEX_SHADER, `
        out vec2 v_position;

        void main() {
            v_position = a_offset;
            gl_Position = vec4(v_position*2.0-1.0, 0, 1);
        }
    `, I_OFFSETS_STRIP);
    const transferFragmentShader = compileShader(gl.FRAGMENT_SHADER, `
        layout(location = 0) out vec4 color;
        in vec2 v_position;
        uniform sampler2D u_tex;

        void main() {
            color = texture(u_tex, v_position);
        }
    `);
    const transferProgram = new Shader(transferVertexShader, transferFragmentShader);
    return (texture, clear = false) => {
        transferProgram.bind();
        gl.uniform1i(transferProgram.uniforms.u_tex, texture.attach(0));
        blit(null, clear);
    }
})();
