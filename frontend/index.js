const count = 3000;
const componentsPerVertex = 2;
const featureDrawingEditor = true;

import initElve, {App, EventKind} from './pkg/elve.js';

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
const shaderConstants = {
    MODE_COLOR: 0,
    MODE_CLEAN_WITH_TRAILS: 1,
    MODE_BLOOM: 2,
    MODE_TEXTURE: 3,
};
const shaderDefines = Object.entries(shaderConstants).map((entry) => `#define ${entry[0]} ${entry[1]}`).join('\n');

const shaderSources = {
    VERTEX_SHADER: `
    precision highp float;
    attribute vec2 aPosition;
    attribute vec4 aColor;
    varying highp vec4 color;
    varying highp vec2 position;
    uniform highp int pass;
    void main() {
        gl_Position = vec4(aPosition, 0.0, 1.0);
        position = aPosition;
        float brightness = aColor.a;
        gl_PointSize = 3.5 * brightness;//pass == 0? (brightness*brightness) * 3.0 + 1.0 : 1.0;
        color = aColor;
    }
    `,
    FRAGMENT_SHADER:  `
    precision highp float;
    uniform sampler2D screen;
    uniform sampler2D prevScreen;
    uniform highp int pass;
    varying highp vec4 color;
    varying highp vec2 position;
    #define kernelSize 3
    ${shaderDefines}
    void main() {
        float a = 0.0002;
        vec2 texPos = vec2((position.x + 1.0) / 2.0, (position.y + 1.0) / 2.0);

        if (pass == MODE_COLOR) {
            gl_FragColor = color;
        } else if (pass == MODE_CLEAN_WITH_TRAILS) {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 0.36);
        } else if (pass == MODE_BLOOM) {
            float a = 0.0028;
            vec4 neighbors;
            vec4 current = texture2D(screen, texPos);


            int count = 0;
            for (int y = -kernelSize; y <= kernelSize; y++) {
                for (int x = -kernelSize; x <= kernelSize; x++) {
                    neighbors += texture2D(screen, texPos + vec2(a * float(x), a * float(y)));// * influence * 0.3;
                    if (x == 0 && y == 0) {
                        neighbors += texture2D(screen, texPos + vec2(a * float(x), a * float(y)));
                        count++;
                    }
                    count++;
                }
            }



            neighbors /= float(count);
            float neighborBrightness = (neighbors.r + neighbors.g + neighbors.b) * neighbors.a;
            float currentBrightness = (current.r + current.g + current.b) * current.a;
            float factor = 3.2;
            if (currentBrightness > neighborBrightness) {
                gl_FragColor = mix(current * factor, neighbors * factor, 0.4);
            } else {
                gl_FragColor = neighbors * factor;//mix(neighbors / 3.0, current, 0.1);//texture2D(screen, texPos);
            }
        } else if (pass == MODE_TEXTURE) {
            gl_FragColor = texture2D(screen, texPos);
        }

    }
    `,
};
const postprocessingShaderSources = {
    VERTEX_SHADER: ``,
    FRAGMENT_SHADER: `
        precision highp float;
        uniform sampler2D prevScreen;
        uniform sampler2D screen;
        void main() {
            vec2 texPos = TODO;!!!!!
            gl_FragColor = mix(texture2D(prevScreen, texPos), texture2D(screen, texPos), 0.5);
        }
    `,
};
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

const shaders = Object.fromEntries(Object.entries(shaderSources).map(([k, source]) => {
    const shader = gl.createShader(gl[k]);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    return [k.split('_')[0].toLowerCase(), shader];
}));

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
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, canvas.width, canvas.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, drawingEditor.layers[i]);
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


