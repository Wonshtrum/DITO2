function unimplemented(name) {
    return () => console.log(name)
}

function get_memory(ptr, len) {
    return new Uint8Array(dito2.instance.exports.memory.buffer.slice(ptr, ptr + len));
}

let dito2;
let memory;
let canvas = document.getElementById("canvas");
let context = canvas.getContext("2d");

async function init() {
    canvas.width = 80 * 4;
    canvas.height = 60 * 4;
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
                context.fillStyle = `rgb(${r}, ${g}, ${b})`;
                context.fillRect(4 * x, 4 * y, 4 * w - 1, 4 * h - 1);
            },
        }
    });
    dito2.instance.exports.init_panic_hook();

    let start = Date.now();
    dito2.instance.exports.main();
    let end = Date.now();
    console.log((end - start) / 1000);
}

init();