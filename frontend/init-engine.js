import initElve, {App} from './pkg/elve.js';

const componentsPerVertex = 2;

export function initEngine(count, init) {
    const pointers = {};
    window.pass_firework_buffers = function (positions, colors) {
        pointers.positions = positions;
        pointers.colors = colors;
    }

    initElve().then(engine => {
        const width = 1024;
        const height = 1024;

        let c=0;
        let lastLength = 0;
        const check = () => {
            if (engine.memory.buffer.byteLength > lastLength) console.log("grown!")
            lastLength = engine.memory.buffer.byteLength;
        }

        const mainApp = new App(width, height);

        const drawingEditor = {};
        drawingEditor.layers = [
            new Uint8Array(engine.memory.buffer, mainApp.drawing_editor_pixels(0), width * height * 4),
            new Uint8Array(engine.memory.buffer, mainApp.drawing_editor_pixels(1), width * height * 4),
            new Uint8Array(engine.memory.buffer, mainApp.drawing_editor_pixels(2), width * height * 4),
        ];
        drawingEditor.width = width;
        drawingEditor.height = height;

        init({
            mainApp,
            buffers: {
                fireworks: {
                    positions: new Float32Array(engine.memory.buffer, pointers.positions, count * componentsPerVertex),
                    colors: new Float32Array(engine.memory.buffer, pointers.colors, count * 4),
                }
            },
            drawingEditor,
        });
    });
}