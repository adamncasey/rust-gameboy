import { WasmGameboy } from "wasm-gameboy";
import { memory } from "wasm-gameboy/wasm_gameboy_bg";

const canvas = document.getElementById("gameboy-canvas");

const gb = WasmGameboy.new([0,0,0,0,0,0]);

canvas.height = gb.buffer_height();
canvas.width = gb.buffer_width();

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    let draw = gb.cycle();

    if (draw) {
        renderScreen(gb, ctx);
    }
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

const renderScreen = (gb, ctx) => {
    const screenPtr = gb.buffer();
    const screenData = new Uint8Array(memory.buffer, screenPtr, gb.buffer_size());
    const image = ImageData(screenData, gb.buffer_width(), gb.buffer_height());
    ctx.putImageData(image, 0, 0);
}