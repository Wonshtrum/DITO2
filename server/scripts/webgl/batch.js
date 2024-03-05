'use strict';


class Batch {
    constructor(layout, maxQuad = 1000) {
        //VERTEX ARRAY
        this.va = gl.createVertexArray();
        gl.bindVertexArray(this.va);

        //GPU BUFFER
        this.vb = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, this.vb);

        //LAYOUT
        this.vertexSize = applyLayout(layout.map(n => [n, gl.FLOAT]), 6) / GL_SIZES[gl.FLOAT]

        //CPU BUFFER
        this.maxQuad = maxQuad;
        this.quadCount = 0;
        this.index = 0;
        this.vertexBuffer = new Float32Array(this.vertexSize * maxQuad);

        //COPY CPU BUFFER TO GPU BUFFER
        gl.bufferData(gl.ARRAY_BUFFER, this.vertexBuffer, gl.DYNAMIC_DRAW);
    }

    draw(...floats) {
        this.vertexBuffer.set(floats, this.index);
        this.index += this.vertexSize;
        this.quadCount += 1;
        if (this.quadCount >= this.maxQuad) {
            this.flush();
        }
    }

    bind() {
        gl.bindVertexArray(this.va);
        gl.bindBuffer(gl.ARRAY_BUFFER, this.vb);
    }

    flush(reset = true) {
        this.bind();
        gl.bufferSubData(gl.ARRAY_BUFFER, 0, this.vertexBuffer.subarray(0, this.index));
        gl.uniform1i(SHADERS.batch.uniforms.u_debug, 0);
        gl.drawArraysInstanced(gl.TRIANGLES, 0, 6, 6 * this.quadCount);
        // gl.uniform1i(SHADERS.batch.uniforms.u_debug, 1);
        // gl.drawArraysInstanced(gl.LINES, 0, 6, 6 * this.quadCount);
        if (reset) {
            this.quadCount = 0;
            this.index = 0;
        }
    }
};

// [(x, y), (w, h), (t), (r, g, b, a)]
const BATCH = new Batch([2, 2, 1, 4]);
