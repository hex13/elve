import {shaderConstants} from './shaders';

export class DrawingEditorRenderer {
    app: any;
    renderer: any;
    gl: any;
    quad: unknown;
    width: number;
    height: number;
    model: any;
    constructor(gl: any, app: any, renderer: any, quad: unknown, w: number, h: number, model: any) {
        this.app = app;
        this.renderer = renderer;
        this.gl = gl;
        this.width = w;
        this.height = h;
        this.quad = quad;
        this.model = model;
    }
    render(shader: any) {
        const gl = this.gl;
        const isDirty = this.app.dirty();
        const layerOrder = [0, 1, 2]; // TODO connect this with GUI!

        gl.uniform1i(shader.uniforms.pass, shaderConstants.MODE_TEXTURE);
        gl.activeTexture(gl.TEXTURE0);
        gl.uniform1i(shader.uniforms.screen, 0);
        const layers = Object.values(this.model.buffers);
        for (let i = 0; i < layers.length; i++) {
            gl.bindTexture(gl.TEXTURE_2D, this.model.textures[i]);
            const layer = layers[layerOrder[i]];
            if (isDirty)
                gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, this.width, this.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, layer);
            this.renderer.render(shader, this.quad);
        }
        this.app.set_dirty(false);
    }
}
