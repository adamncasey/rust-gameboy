import { WasmGameboy } from "wasm-gameboy";
import { memory } from "wasm-gameboy/wasm_gameboy_bg";

let gb = undefined;
let ctx = undefined;

let debug_state = { "enabled": false, "stopping": false, "stopped": true, "stop_handler": () => { } };

const initialiseGameboy = (rom) => {
    const canvas = document.getElementById("gameboy-canvas");

    gb = WasmGameboy.new(rom.byteLength);

    const cartridgeData = new Uint8Array(rom, 0, rom.byteLength);

    // Load ROM
    let romView = new Uint8Array(memory.buffer, gb.rom_buffer(), rom.byteLength);
    console.log(rom);
    romView.set(cartridgeData);
    console.log(romView);
    gb.start();

    canvas.height = gb.screen_height();
    canvas.width = gb.screen_width();

    ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;

    startPlayLoop();
};

const renderScreen = (gb, ctx) => {
    const screenPtr = gb.screen_buffer();
    const screenData = new Uint8ClampedArray(memory.buffer, screenPtr, gb.screen_size());
    const image = new ImageData(screenData, gb.screen_width(), gb.screen_height());
    ctx.putImageData(image, 0, 0);
}

const startPlayLoop = () => {
    debug_state.stopped = false;
    debug_state.stopping = false;
    updateDebugRunControls();

    requestAnimationFrame(normalPlayLoop);
}

const normalPlayLoop = () => {
    let render = gb.cycle_until_vsync();

    if (render) {
        renderScreen(gb, ctx);
        fps.render(1);
    }

    if (debug_state.stopping === false) {
        requestAnimationFrame(normalPlayLoop);
    } else {
        debug_state.stopped = true;
        debug_state.stopping = false;
        debug_state.stop_handler();
    }
}

const singleStep = () => {
    let render = gb.cycle();

    if (render) {
        renderScreen(gb, ctx);
        fps.render(1);
    }
}

function updateDebugInfo() {
    if (gb) {
        let info = gb.debug_info();
        console.log(info);

        document.getElementById("cpu-pc").innerText = info.pc;
        document.getElementById("cpu-sp").innerText = info.sp;
        document.getElementById("cpu-a").innerText = info.a;
        document.getElementById("cpu-b").innerText = info.b;
        document.getElementById("cpu-c").innerText = info.c;
        document.getElementById("cpu-d").innerText = info.d;
        document.getElementById("cpu-e").innerText = info.e;
        document.getElementById("cpu-f").innerText = info.f;
        document.getElementById("cpu-h").innerText = info.h;
        document.getElementById("cpu-l").innerText = info.l;

        updateDisassembly(info.pc);
    }
}

function updateDisassembly(pc) {
    const disassembly = gb.disassemble(Math.max(pc - 1000, 0), Math.min(pc + 1000, 0xFFFF));
    console.log({disassembly});

    let table = document.createElement("table");

    for (let instr of disassembly) {
        let row = document.createElement("tr");

        let addr_td = document.createElement("td");
        var addr = document.createTextNode(instr.addr);
        addr_td.appendChild(addr);
        row.appendChild(addr_td);

        let desc_td = document.createElement("td");
        var desc = document.createTextNode(instr.desc);
        desc_td.appendChild(desc);
        row.appendChild(desc_td);

        table.appendChild(row);
    }

    let container = document.getElementById("disassembly");
    container.innerHTML = '';
    container.appendChild(table);
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
        for (let i = 0; i < this.frames.length; i++) {
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }

        // Render the statistics.
        this.fps.textContent = `${Math.round(min)}~${Math.round(max)} Hz`.trim();
    }
};

const updateDebugRunControls = () => {
    if (debug_state.stopped) {
        document.getElementById("pause").disabled = true;
        document.getElementById("play").disabled = false;
        document.getElementById("step").disabled = false;
    } else {
        document.getElementById("pause").disabled = false;
        document.getElementById("play").disabled = true;
        document.getElementById("step").disabled = true;
    }
};

document.getElementById("debug-toggle").addEventListener("click", function (evt) {
    let hide = document.getElementById("debug-toggle-off");
    let show = document.getElementById("debug-toggle-on");
    let debug_panel = document.getElementById("debug-view");

    if (hide.style.display == "none") {
        hide.style.display = "inline-block";
        show.style.display = "none";
        debug_panel.style.display = "block";
        debug_state.enabled = true;
    }
    else {
        hide.style.display = "none";
        show.style.display = "inline-block";
        debug_panel.style.display = "none";
        debug_state.enabled = false;
    }
});


document.getElementById("pause").addEventListener("click", function (evt) {
    debug_state.stop_handler = () => {
        updateDebugInfo();
        updateDebugRunControls();
    };
    debug_state.stopping = true;
});

document.getElementById("step").addEventListener("click", function (evt) {
    if (gb && debug_state.stopped === true) {
        singleStep();
        updateDebugInfo();
    }
});

document.getElementById("play").addEventListener("click", function (evt) {
    if (gb && debug_state.stopped === true) {
        startPlayLoop();
    }
});