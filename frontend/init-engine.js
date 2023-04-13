import initElve, {App} from './pkg/elve.js';

const componentsPerVertex = 2;

export function initEngine(count, init) {
    const pointers = {};
    const schemas = [
        ['positions', count * componentsPerVertex],
        ['colors', count * 4],
    ];

    window.pass_buffer = function (index, pointer) {
        const [key] = schemas[index];
        pointers[key] = pointer;
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

        const fireworks = {};
        Object.entries(pointers).forEach(([key, pointer]) => {
            const [name, length] = schemas.find(schema => schema[0] == key)
            fireworks[key] = new Float32Array(engine.memory.buffer, pointer, length);
        });

        init({
            mainApp,
            buffers: {
                fireworks,
            },
            drawingEditor,
        });
    });
}