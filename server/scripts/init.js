'use strict';


let DITO2;
let WORLD;
let ATLAS;

// let mainFBO = new FBO(RESOLUTION, RESOLUTION, [AP("main")]);

function unimplemented(name) {
    return () => console.log(name)
}

function get_memory(ptr, len) {
    return new Uint8Array(DITO2.memory.buffer.slice(ptr, ptr + len));
}

async function init() {
    ATLAS = loadTexture("atlas.png");
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
            "draw_quad": (x, y, w, h, tex, r, g, b, a) => {
                BATCH.draw(x, y, w, h, tex, r / 256, g / 256, b / 256, a / 256);
            },
            "new_mesh": (ptr, len) => {
                return CHUNKER.new(get_memory(ptr, len));
            },
            "update_mesh": (id, ptr, len) => {
                CHUNKER.update(id, get_memory(ptr, len));
            },
            "free_mesh": (id) => {
                CHUNKER.free(id);
            },
        }
    });
    DITO2 = dito2.instance.exports;
    DITO2.init_panic_hook();
    WORLD = DITO2.create_world();
    DITO2.debug(WORLD);

    gl.clearColor(0, 0, 0, 0);
    unbindAllFBO();
    tick();
}

let x = 0;
let y = 0;
let ticks = 0;
let SPEED = 2;
let ZOOM = 8;
function set_camera(shader) {
    shader.bind();
    gl.uniform2f(shader.uniforms.u_screen, RESOLUTION, RESOLUTION);
    gl.uniform3f(shader.uniforms.u_camera, x, y, ZOOM);
    gl.uniform1i(shader.uniforms.u_t, ticks);
}

let BLOCK_ID = 0;
let BLOCK_FLAGS = 0;
let acc_time = Array(10);
function tick() {
    let start = Date.now();

    set_camera(SHADERS.batch);
    gl.clear(gl.COLOR_BUFFER_BIT);
    // DITO2.debug_draw(WORLD);
    DITO2.update(WORLD);
    CHUNKER.draw_all();
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

    let time = Date.now() - start;
    acc_time.splice(0, 1);
    acc_time.push(time);
    let avg = acc_time.sum() / acc_time.length;
    CANVAS_INFOS.innerHTML = `${avg.toFixed(1)}`;
    ticks += 1;
    requestAnimationFrame(tick);
}

init();
