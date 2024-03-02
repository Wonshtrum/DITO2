'use strict';


function initWebGL(canvas) {
    let gl = canvas.getContext("webgl2", { preserveDrawingBuffer: true, premultipliedAlpha: false });
    gl.enable(gl.BLEND);
    return gl;
}

function additiveBlend(active) {
    if (active) {
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE);
    } else {
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    }
}

const CANVAS = document.getElementById("canvas");
const gl = initWebGL(CANVAS);
let RESOLUTION = 800;
CANVAS.width = RESOLUTION;
CANVAS.height = RESOLUTION;
