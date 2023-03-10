import {createShaders} from './shaders';

export class Renderer {
    init(canvas) {
        const gl = canvas.getContext('webgl2', {
            premultipliedAlpha: false,
            alpha: true,
            antialias: true,
            preserveDrawingBuffer: true,
        });
        gl.enable(gl.BLEND);
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

        const program = gl.createProgram();
        const shaders = createShaders(gl);

        gl.attachShader(program, shaders.fragment);
        gl.attachShader(program, shaders.vertex);
        gl.linkProgram(program);

        this.gl = gl;
        this.program = program;

        return this;
    }
}