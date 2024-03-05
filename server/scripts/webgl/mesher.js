'use strict';


class Mesher {
    constructor(layout, reservedSize) {
        this.layout = layout;
        this.vertexSize = layoutVertexSize(layout);
        this.reservedSize = reservedSize;
        this.reservedSpace = new Uint8Array(this.vertexSize * reservedSize);
        this.chunk_id = 1;
        this.chunks = {}
    }

    new(vertexBuffer) {
        console.log("Mesh::new");
        //VERTEX ARRAY
        let va = gl.createVertexArray();
        gl.bindVertexArray(va);

        //GPU BUFFER
        let vb = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, vb);

        //LAYOUT
        applyLayout(this.layout, 6, this.vertexSize);

        //COPY CPU BUFFER TO GPU BUFFER
        gl.bufferData(gl.ARRAY_BUFFER, this.reservedSpace, gl.DYNAMIC_DRAW);
        gl.bufferSubData(gl.ARRAY_BUFFER, 0, vertexBuffer);

        let vertexCount = 6 * vertexBuffer.length / this.vertexSize;
        this.chunks[this.chunk_id] = { va, vb, vertexCount };
        return this.chunk_id++;
    }

    update(chunk_id, vertexBuffer) {
        console.log("Mesh::update");
        let chunk = this.chunks[chunk_id];
        gl.bindBuffer(gl.ARRAY_BUFFER, chunk.vb);
        gl.bufferSubData(gl.ARRAY_BUFFER, 0, vertexBuffer);
        chunk.vertexCount = 6 * vertexBuffer.length / this.vertexSize;
    }

    free(chunk_id) {
        console.log("Mesh::free");
        let chunk = this.chunks[chunk_id];
        gl.deleteVertexArray(chunk.va)
        gl.deleteBuffer(chunk.vb);
    }

    draw(chunk_id) {
        let chunk = this.chunks[chunk_id];
        gl.bindVertexArray(chunk.va);
        gl.bindBuffer(gl.ARRAY_BUFFER, chunk.vb);
        gl.drawArraysInstanced(gl.TRIANGLES, 0, 6, chunk.vertexCount);
    }

    draw_all() {
        for (let chunk of Object.values(this.chunks)) {
            gl.bindVertexArray(chunk.va);
            gl.bindBuffer(gl.ARRAY_BUFFER, chunk.vb);
            gl.drawArraysInstanced(gl.TRIANGLES, 0, 6, chunk.vertexCount);
        }
    }
};


// [(x, y), (w, h), (t), (r, g, b, a)]
const CHUNKER = new Mesher([[2, gl.FLOAT], [2, gl.FLOAT], [1, gl.FLOAT], [4, gl.UNSIGNED_BYTE, true]], 16 * 16);
