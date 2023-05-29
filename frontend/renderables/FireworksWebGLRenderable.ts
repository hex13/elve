import {shaderConstants} from '../shaders';

const count = 3000;
const componentsPerVertex = 2;

export class FireworksWebGLRenderable {
    gl: any;
    buffer: unknown;
    colorBuffer: unknown;
    model: any;
    renderer: any;
    texture: unknown;
    constructor(gl: any, app: any, renderer: any, w: number, h: number, model: any) {
        this.gl = gl;
        this.buffer = gl.createBuffer();
        this.colorBuffer = gl.createBuffer();
        this.model = model;
        this.renderer = renderer;
        this.texture = renderer.createTexture(w, h);
    }
    render(shader: any) {
        const {gl, renderer, texture } = this;
        const { uniforms } = shader;
        renderer.renderTo(texture, () => {
            gl.uniform1i(uniforms.pass, shaderConstants.MODE_CLEAN_WITH_TRAILS);
            renderer.render(shader, renderer.quad);
            this._render(shader);
        });

        gl.uniform1i(uniforms.pass, shaderConstants.MODE_BLOOM);
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.uniform1i(uniforms.screen, 0);
        renderer.render(shader, renderer.quad);
    }

    _render(shader: any) {
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
