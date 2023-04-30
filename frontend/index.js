// TODO pass count to Wasm
const count = 3000;

import {EventKind} from './pkg/elve.js';
import {shaderConstants} from './shaders';
import {initEngine} from './init-engine';
import {createEventHandlers} from './canvas-events.js';
import {Renderer} from './renderer.js';
import {createShader} from './shaders';
import {FireworksRenderer} from './fireworks';
import {DrawingEditorRenderer} from './drawingEditor';

import * as gui from './gui/gui';

initEngine(count, init);
function init({ mainApp, models, drawingEditor }) {
    const canvas = document.getElementById('game');
    const handlers = createEventHandlers(canvas, mainApp, drawingEditor);

    canvas.addEventListener('pointerdown', handlers.pointerDown);
    canvas.addEventListener('pointermove', handlers.pointerMove);
    canvas.addEventListener('pointerup', handlers.pointerUp);
    canvas.addEventListener('touchstart', e => {
        e.preventDefault();
    });

    let layerOrder = [0, 1, 2];

    gui.createGUI(document.getElementById('gui'), {
        changeLayers(layers) {
            layerOrder = layers.map(layer => layer.id);
        },
        ...handlers,
    });

const renderer = new Renderer();
const { gl } = renderer.init(canvas);
const shader = createShader(gl);
const { program, uniforms, attributes } = shader;





gl.useProgram(program);

let texture = renderer.createTexture(canvas.width, canvas.height);
models[1].textures = Object.values(models[1].buffers).map(() => renderer.createTexture(canvas.width, canvas.height));

const rendererConstructors = {
    fireworks: FireworksRenderer,
    drawingEditor: DrawingEditorRenderer,
};
const views = models.map(model => {
    let Constr = Object.hasOwn(rendererConstructors, model.name)? rendererConstructors[model.name] : null;
    return {renderer: Constr? new Constr(gl, mainApp, renderer, canvas.width, canvas.height, model) : null};
});

let fps = 0;
let maxFpsCounter = 100;
let lastFpsTime = Date.now();
const fpsEl = document.getElementById('fps');
(function update() {
    mainApp.update();

    renderer.renderTo(texture, () => {
        gl.uniform1i(uniforms.pass, shaderConstants.MODE_CLEAN_WITH_TRAILS);
        renderer.render(shader, renderer.quad);
        views[0].renderer.render(shader);
    });

    gl.uniform1i(uniforms.pass, shaderConstants.MODE_BLOOM);
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.uniform1i(uniforms.screen, 0);
    renderer.render(shader, renderer.quad);

    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, null);
    views[1].renderer.render(shader);

    fps++;
    if (fps == maxFpsCounter) {
        fps = 0;
        const now = Date.now();

        fpsEl.innerText = maxFpsCounter / (now - lastFpsTime) * 1000;
        lastFpsTime = now;
    }
    requestAnimationFrame(update);
})();

}

