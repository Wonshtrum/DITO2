'use strict';


let DITO2;
let WORLD;

// let mainFBO = new FBO(RESOLUTION, RESOLUTION, [AP("main")]);

function unimplemented(name) {
    return () => console.log(name)
}

function get_memory(ptr, len) {
    return new Uint8Array(DITO2.memory.buffer.slice(ptr, ptr + len));
}

async function init() {
    let dito2_path = "../target/wasm32-unknown-unknown/release/dito2.wasm";
    let dito2 = await WebAssembly.instantiateStreaming(fetch(dito2_path), {
        "dito2": {
            "log": (ptr, len) => {
                let msg = new TextDecoder().decode(get_memory(ptr, len));
                console.log(msg);
            },
            "error": (ptr, len) => {
                let msg = new TextDecoder().decode(get_memory(ptr, len));
                console.error(msg);
            },
            "fill_rect": (x, y, w, h, r, g, b) => {
                BATCH.draw(x, y, w - 1 / 8, h - 1 / 8, r / 256, g / 256, b / 256, 1);
            }
        }
    });
    DITO2 = dito2.instance.exports;
    DITO2.init_panic_hook();
    WORLD = DITO2.create_world();
    DITO2.debug(WORLD);
    tick();
}

let x = 0;
let y = 0;
let SPEED = 2;
let ZOOM = 8;

let BLOCK_ID = 0;
let BLOCK_FLAGS = 0;
function tick() {
    unbindAllFBO();
    SHADERS.batch.bind();
    gl.uniform2f(SHADERS.batch.uniforms.u_screen, RESOLUTION, RESOLUTION);
    gl.uniform3f(SHADERS.batch.uniforms.u_camera, x, y, ZOOM);
    gl.clearColor(1, 1, 1, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    DITO2.update(WORLD);
    BATCH.flush();
    // transferTarget(mainFBO.texture);

    if (KEYS[ARROW_UP] || KEYS["Z"]) {
        y += SPEED;
    }
    if (KEYS[ARROW_DOWN] || KEYS["S"]) {
        y -= SPEED;
    }
    if (KEYS[ARROW_RIGHT] || KEYS["D"]) {
        x += SPEED;
    }
    if (KEYS[ARROW_LEFT] || KEYS["Q"]) {
        x -= SPEED;
    }

    if (CURSOR.buttons === 1) {
        let block_x = x + (CURSOR.x * 2 - 1) * RESOLUTION / ZOOM;
        let block_y = y - (CURSOR.y * 2 - 1) * RESOLUTION / ZOOM;
        DITO2.set_block(WORLD, block_x, block_y, BLOCK_ID, BLOCK_FLAGS)
    }
    requestAnimationFrame(tick);
}

init();
