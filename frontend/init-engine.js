import initElve, {App} from './pkg/elve.js';

const componentsPerVertex = 2;

export function initEngine(count, init, config) {
    const { models } = config;
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

        const mainApp = new App(width, height, false);

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