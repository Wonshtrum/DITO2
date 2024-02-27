'use strict';


let dito2;

// let mainFBO = new FBO(RESOLUTION, RESOLUTION, [AP("main")]);

function unimplemented(name) {
    return () => console.log(name)
}

function get_memory(ptr, len) {
    return new Uint8Array(dito2.instance.exports.memory.buffer.slice(ptr, ptr + len));
}

async function init() {
    let dito2_path = "../target/wasm32-unknown-unknown/release/dito2.wasm";
    dito2 = await WebAssembly.instantiateStreaming(fetch(dito2_path), {
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
                let zoom = 8;
                BATCH.draw(zoom * x / 512, zoom * y / 512, (zoom * w - 1) / 512, (zoom * h - 1) / 512, r / 256, g / 256, b / 256, 1);
            }
        }
    });
    dito2.instance.exports.init_panic_hook();

    let start = Date.now();

    unbindAllFBO();
    SHADERS.batch.bind();
    dito2.instance.exports.main();
    BATCH.flush();
    // transferTarget(mainFBO.texture);

    let end = Date.now();
    console.log((end - start) / 1000);
}

init();
