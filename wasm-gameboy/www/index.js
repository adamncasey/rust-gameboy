import { WasmGameboy } from "wasm-gameboy";
import { memory } from "wasm-gameboy/wasm_gameboy_bg";

let frame_count = 0;

let gb = undefined;

const initialiseGameboy = (rom) => {
    const canvas = document.getElementById("gameboy-canvas");

    gb = WasmGameboy.new(rom.byteLength);
    gb.debug(document.getElementById("debug").checked);

    const cartridgeData = new Uint8Array(rom, 0, rom.byteLength);

    // Load ROM
    let romView = new Uint8Array(memory.buffer, gb.rom_buffer(), rom.byteLength);
    console.log(rom);
    romView.set(cartridgeData);
    console.log(romView);
    gb.start();

    canvas.height = gb.screen_height();
    canvas.width = gb.screen_width();

    const ctx = canvas.getContext('2d');

    const renderLoop = () => {
        let render = gb.cycle_until_vsync();

        if (render) {
            frame_count += 1;

            renderScreen(gb, ctx);
            fps.render(1);
        }

        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
};

const renderScreen = (gb, ctx) => {
    const screenPtr = gb.screen_buffer();
    const screenData = new Uint8ClampedArray(memory.buffer, screenPtr, gb.screen_size());
    const image = new ImageData(screenData, gb.screen_width(), gb.screen_height());
    ctx.putImageData(image, 0, 0);
}

document.getElementById("insert-cartridge").addEventListener('change', (evt) => {
    const file = evt.target.files[0];

    const reader = new FileReader();
    reader.onload = (evt) => {
        console.log({ evt });
        initialiseGameboy(evt.target.result);
    };
    reader.readAsArrayBuffer(file);

});

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render(numCycles) {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = numCycles / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `
  CPU Emulation Hz:
           latest = ${Math.round(fps)}
  avg of last 100 = ${Math.round(mean)}
  min of last 100 = ${Math.round(min)}
  max of last 100 = ${Math.round(max)}
  `.trim();
    }
};

document.getElementById("debug").addEventListener("change", function (evt) {
    if (gb) {
        gb.debug(document.getElementById("debug").checked);
    }
});