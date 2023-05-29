import initElve, {App} from './pkg/elve.js';

export function initEngine(count, init, config) {
    const { models } = config;
    const { width, height } = config.size;
    window.pass_buffer = function (modelIndex, bufferIndex, pointer, length) {
        const model = models[modelIndex];
        const key = model.bufferNames[bufferIndex];
        model.pointers[key] = pointer;
        model.lengths[key] = length;
    }

    initElve().then(engine => {
        let lastLength = 0;
        const check = () => {
            if (engine.memory.buffer.byteLength > lastLength) console.log("grown!")
            lastLength = engine.memory.buffer.byteLength;
        }

        const mainApp = new App();
        mainApp.add_fireworks_model();
        mainApp.add_drawing_editor_model(width, height);
        // mainApp.add_extra_model(width, height);
        mainApp.init(width, height, true);

        models.forEach(model => {
            Object.entries(model.pointers).forEach(([key, pointer]) => {
                const length = model.lengths[key];
                model.buffers[key] = new model.BufferType(engine.memory.buffer, pointer, length);
            });
        });
        init({
            mainApp,
            models,
        });
    });
}