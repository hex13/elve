import initElve, {App} from './pkg/elve.js';

const componentsPerVertex = 2;

export function initEngine(count, init) {
    const models = [
        {
            name: 'fireworks',
            bufferNames: [
                'positions',
                'colors',
            ],
            BufferType: Float32Array,
            pointers: {},
            lengths: {},
            buffers: {}
        },
        {
            name: 'drawingEditor',
            bufferNames: [
                'layer0',
                'layer1',
                'layer2',
            ],
            BufferType: Uint8Array,
            pointers: {},
            lengths: {},
            buffers: {}
        },
        {
            name: 'drawingEditor',
            bufferNames: [
                'layer0',
                'layer1',
                'layer2',
            ],
            BufferType: Uint8Array,
            pointers: {},
            lengths: {},
            buffers: {}
        },
    ];

    window.pass_buffer = function (modelIndex, bufferIndex, pointer, length) {
        const model = models[modelIndex];
        const key = model.bufferNames[bufferIndex];
        model.pointers[key] = pointer;
        model.lengths[key] = length;
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
        drawingEditor.width = width;
        drawingEditor.height = height;

        const fireworks = {};

        models.forEach(model => {
            Object.entries(model.pointers).forEach(([key, pointer]) => {
                const length = model.lengths[key];
                model.buffers[key] = new model.BufferType(engine.memory.buffer, pointer, length);
            });
        });
        init({
            mainApp,
            models,
            drawingEditor,
        });
    });
}