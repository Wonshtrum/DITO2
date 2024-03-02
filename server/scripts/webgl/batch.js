'use strict';


class Batch {
    constructor(layout, maxQuad = 1000) {
        //VERTEX ARRAY
        this.va = gl.createVertexArray();
        gl.bindVertexArray(this.va);

        //CPU BUFFER
        this.vertexSize = layout.sum();
        this.maxQuad = maxQuad;
        this.quadCount = 0;
        this.index = 0;
        this.vertexBuffer = new Float32Array(this.vertexSize * maxQuad);

        //GPU BUFFER
        this.vb = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, this.vb);
        gl.bufferData(gl.ARRAY_BUFFER, this.vertexBuffer, gl.DYNAMIC_DRAW);

        //LAYOUT
        apply_layout(layout, 6);
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
        gl.drawArraysInstanced(gl.TRIANGLES, 0, 6, 6 * this.quadCount);
        if (reset) {
            this.quadCount = 0;
            this.index = 0;
        }
    }
};

// [(x, y), (w, h), (r, g, b, a)]
const BATCH = new Batch([2, 2, 4]);
