import {shaderConstants} from './shaders';

const count = 3000;
const componentsPerVertex = 2;

export class FireworksRenderer {
    gl: any;
    buffer: unknown;
    colorBuffer: unknown;
    model: any;
    constructor(gl: any, model: any) {
        this.gl = gl;
        this.buffer = gl.createBuffer();
        this.colorBuffer = gl.createBuffer();
        this.model = model;
    }
    render(shader: any) {
        const gl = this.gl;
        gl.uniform1i(shader.uniforms.pass, shaderConstants.MODE_COLOR);
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, null);
        gl.activeTexture(gl.TEXTURE1);
        gl.bindTexture(gl.TEXTURE_2D, null);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.buffer);
        gl.bufferData(gl.ARRAY_BUFFER, this.model.buffers.positions, gl.DYNAMIC_DRAW);

        const aPosition = shader.attributes.aPosition;
        gl.vertexAttribPointer(aPosition, componentsPerVertex, gl.FLOAT, false, 0, 0);
        gl.enableVertexAttribArray(aPosition);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.colorBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, this.model.buffers.colors, gl.DYNAMIC_DRAW);

        const aColor = shader.attributes.aColor;

        gl.vertexAttribPointer(aColor, 4 /*rgba*/, gl.FLOAT, false, 0, 0);
        gl.enableVertexAttribArray(aColor);

        gl.drawArrays(gl.POINTS, 0, count);
    }
}
