const count = 3000;
const componentsPerVertex = 2;
const featureDrawingEditor = true;

import initElve, {App, EventKind} from './pkg/elve.js';
import {createShaders, shaderConstants} from './shaders';
import * as gui from './gui';

let layerOrder = [0, 1, 2];
gui.createGUI(document.getElementById('gui'), {
    changeLayers(layers) {
        layerOrder = layers.map(layer => layer.id);
    }
});
let wasmPositions, colors;

let drawingEditor;
let mainApp;
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

    drawingEditor = {};
    drawingEditor.layers = [
        new Uint8Array(engine.memory.buffer, mainApp.drawing_editor_pixels(0), width * height * 4),
        new Uint8Array(engine.memory.buffer, mainApp.drawing_editor_pixels(1), width * height * 4),
        new Uint8Array(engine.memory.buffer, mainApp.drawing_editor_pixels(2), width * height * 4),
    ];
    drawingEditor.width = width;
    drawingEditor.height = height;

    const buff = new Float32Array(engine.memory.buffer, pointers.positions, count * componentsPerVertex);
    wasmPositions = buff;

    colors = new Float32Array(engine.memory.buffer, pointers.colors, count * 4);


    init(mainApp);
});


function init(mainApp) {

    function check({shader, program} = {}) {
    const err = gl.getError();
    if (err) console.log("ERR",err);
    if (shader) {
        const info = gl.getShaderInfoLog(shader);
        if (info) console.log(info)
    }
    if (program) {
        const info = gl.getProgramInfoLog(program);
        if (info) console.log(info);
    }
}
const canvas = document.getElementById('game');
canvas.addEventListener('pointerdown', e => {
    const bounds = e.target.getBoundingClientRect();
    const canvasX = e.clientX - bounds.left;
    const canvasY = e.clientY - bounds.top;

    const scaleX = bounds.width / drawingEditor.width;
    const scaleY = bounds.height / drawingEditor.height;
    mainApp.dispatch(EventKind.PointerDown, canvasX / scaleX, canvasY / scaleY);

});

canvas.addEventListener('touchstart', e => {
    e.preventDefault();
});

canvas.addEventListener('pointermove', e => {
    const bounds = e.target.getBoundingClientRect();
    const canvasX = e.clientX - bounds.left;
    const canvasY = e.clientY - bounds.top;

    const scaleX = bounds.width / drawingEditor.width;
    const scaleY = bounds.height / drawingEditor.height;
    mainApp.dispatch(EventKind.PointerMove, canvasX / scaleX, canvasY / scaleY);
});

canvas.addEventListener('pointerup', e => {
    const bounds = e.target.getBoundingClientRect();
    const canvasX = e.clientX - bounds.left;
    const canvasY = e.clientY - bounds.top;

    const scaleX = bounds.width / drawingEditor.width;
    const scaleY = bounds.height / drawingEditor.height;
    mainApp.dispatch(EventKind.PointerUp, canvasX / scaleX, canvasY / scaleY);

});


const gl = canvas.getContext('webgl2', {
    premultipliedAlpha: false, 
    alpha: true,
    antialias: true,
    preserveDrawingBuffer: true,
});

const shaders = createShaders(gl);

gl.enable(gl.BLEND);
gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
const program = gl.createProgram();
gl.attachShader(program, shaders.fragment);
gl.attachShader(program, shaders.vertex);
gl.linkProgram(program);


const buffer = gl.createBuffer();
const colorBuffer = gl.createBuffer();

const quad = (() => {
    const buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
        -1.0, -1.0,
        1.0, -1.0,
        1.0, 1.0,
        1.0, 1.0,
        -1.0, 1.0,
        -1.0, -1.0,
    ]), gl.DYNAMIC_DRAW);
    return {buffer};
})();

gl.useProgram(program);

function createTexture(width, height) {
    const texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    return texture;
}

let texture = createTexture(canvas.width, canvas.height);
let prevTexture = createTexture(canvas.width, canvas.height);
let nextTexture = createTexture(canvas.width, canvas.height);
let tmpTexture = createTexture(canvas.width, canvas.height);

const drawingTexture = createTexture(canvas.width, canvas.height);

const textureFramebuffer = gl.createFramebuffer();



const velocities = new Float32Array(count * componentsPerVertex);
const particles =  new Float32Array(count * componentsPerVertex);
let center;
function renderQuad() {
    gl.uniform1i(uniforms.screen, 0);
    gl.uniform1i(uniforms.prevScreen, 1);


    gl.bindBuffer(gl.ARRAY_BUFFER, quad.buffer);


    const aPosition = gl.getAttribLocation(program, 'aPosition');
    gl.vertexAttribPointer(aPosition, 2, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(aPosition);

    gl.drawArrays(gl.TRIANGLES, 0, 6);

}
function render() {


    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, null);
    gl.activeTexture(gl.TEXTURE1);
    gl.bindTexture(gl.TEXTURE_2D, null);


    const particles = wasmPositions;


    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, particles, gl.DYNAMIC_DRAW);

    const aPosition = gl.getAttribLocation(program, 'aPosition');
    gl.vertexAttribPointer(aPosition, componentsPerVertex, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(aPosition);


    gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, colors, gl.DYNAMIC_DRAW);

    const aColor = gl.getAttribLocation(program, 'aColor');

    gl.vertexAttribPointer(aColor, 4 /*rgba*/, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(aColor);


    gl.drawArrays(gl.POINTS, 0, count);


}
let angle = 0.0;

const passLocation = gl.getUniformLocation(program, 'pass');
const uniforms = {
    screen: gl.getUniformLocation(program, 'screen'),
    prevScreen: gl.getUniformLocation(program, 'prevScreen'),
};

function renderDrawingEditor() {
    if (!featureDrawingEditor) return;
    gl.uniform1i(passLocation, shaderConstants.MODE_TEXTURE);
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, drawingTexture);


    for (let i = 0; i < drawingEditor.layers.length; i++) {
        const layer = drawingEditor.layers[layerOrder[i]];
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, canvas.width, canvas.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, layer);
        renderQuad();
    }
}
let fps = 0;
let maxFpsCounter = 100;
let lastFpsTime = Date.now();
const fpsEl = document.getElementById('fps');
(function update() {
    mainApp.update();

    gl.bindFramebuffer(gl.FRAMEBUFFER, textureFramebuffer);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);


    gl.uniform1i(passLocation, shaderConstants.MODE_CLEAN_WITH_TRAILS);

    renderQuad();
    gl.uniform1i(passLocation, shaderConstants.MODE_COLOR);

    render();

    renderDrawingEditor();

    gl.uniform1i(passLocation, shaderConstants.MODE_BLOOM);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, null, 0);            
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);

    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, texture);

    renderQuad();
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, null);

    fps++;
    if (fps == maxFpsCounter) {
        fps = 0;
        const now = Date.now();

        fpsEl.innerText = maxFpsCounter / (now - lastFpsTime) * 1000;
        lastFpsTime = now;
    }
    requestAnimationFrame(update);
})();

document.getElementById('autoexplosions').addEventListener('click', e => {
    mainApp.dispatch(EventKind.TogglePlay, 0, 0);
});

document.querySelectorAll('[data-controller]').forEach(el => {
    el.addEventListener('click', e => {
        mainApp.set_controller(~~e.target.getAttribute('data-controller'));
    })
})


}



