// TODO pass count to Wasm
const count = 3000;
const componentsPerVertex = 2;
const featureDrawingEditor = true;

import {EventKind} from './pkg/elve.js';
import {shaderConstants} from './shaders';
import {initEngine} from './init-engine';
import {createEventHandlers} from './canvas-events.js';
import {Renderer} from './renderer.js';
import {createProgram} from './shaders';

import * as gui from './gui/gui';

initEngine(count, init);
function init({ mainApp, wasmPositions, drawingEditor, colors }) {
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
const { program, uniforms, attributes } = createProgram(gl);

const buffer = gl.createBuffer();
const colorBuffer = gl.createBuffer();

const quad = renderer.createBuffer(new Float32Array([
    -1.0, -1.0,
    1.0, -1.0,
    1.0, 1.0,
    1.0, 1.0,
    -1.0, 1.0,
    -1.0, -1.0,
]));

gl.useProgram(program);

let texture = renderer.createTexture(canvas.width, canvas.height);

drawingEditor.textures = drawingEditor.layers.map(() => renderer.createTexture(canvas.width, canvas.height));

const textureFramebuffer = gl.createFramebuffer();


function renderQuad() {
    gl.uniform1i(uniforms.screen, 0);
    gl.uniform1i(uniforms.prevScreen, 1);


    gl.bindBuffer(gl.ARRAY_BUFFER, quad);


    const aPosition = attributes.aPosition;
    gl.vertexAttribPointer(aPosition, 2, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(aPosition);

    gl.drawArrays(gl.TRIANGLES, 0, 6);

}
function renderFireworks() {

    gl.uniform1i(uniforms.pass, shaderConstants.MODE_COLOR);
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, null);
    gl.activeTexture(gl.TEXTURE1);
    gl.bindTexture(gl.TEXTURE_2D, null);


    const particles = wasmPositions;


    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, particles, gl.DYNAMIC_DRAW);

    const aPosition = attributes.aPosition;
    gl.vertexAttribPointer(aPosition, componentsPerVertex, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(aPosition);


    gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, colors, gl.DYNAMIC_DRAW);

    const aColor = attributes.aColor;

    gl.vertexAttribPointer(aColor, 4 /*rgba*/, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(aColor);


    gl.drawArrays(gl.POINTS, 0, count);


}

function renderDrawingEditor() {
    if (!featureDrawingEditor) return;
    const isDirty = mainApp.dirty();

    gl.uniform1i(uniforms.pass, shaderConstants.MODE_TEXTURE);
    gl.activeTexture(gl.TEXTURE0);


    for (let i = 0; i < drawingEditor.layers.length; i++) {
        gl.bindTexture(gl.TEXTURE_2D, drawingEditor.textures[i]);

        const layer = drawingEditor.layers[layerOrder[i]];
        if (isDirty)
            gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, canvas.width, canvas.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, layer);
        renderQuad();
    }
    mainApp.set_dirty(false);

}
let fps = 0;
let maxFpsCounter = 100;
let lastFpsTime = Date.now();
const fpsEl = document.getElementById('fps');
(function update() {
    mainApp.update();

    gl.bindFramebuffer(gl.FRAMEBUFFER, textureFramebuffer);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);


    gl.uniform1i(uniforms.pass, shaderConstants.MODE_CLEAN_WITH_TRAILS);

    renderQuad();

    renderFireworks();


    gl.uniform1i(uniforms.pass, shaderConstants.MODE_BLOOM);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, null, 0);            
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);

    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, texture);

    renderQuad();
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, null);
    renderDrawingEditor();
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

