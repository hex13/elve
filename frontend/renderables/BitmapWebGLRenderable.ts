import {shaderConstants} from '../shaders';

export class BitmapWebGLRenderable {
    app: any;
    renderer: any;
    gl: any;
    width: number;
    height: number;
    model: any;
    texture: unknown;
    constructor(gl: any, app: any, renderer: any, w: number, h: number, model: any) {
        this.app = app;
        this.renderer = renderer;
        this.gl = gl;
        console.log("bitmap renderable", model)
        this.width = model.width;
        this.height = model.height;
        this.model = model;
        this.texture = renderer.createTexture(model.width, model.height);
    }
    render(shader: any) {
        const gl = this.gl;
        const isDirty = true;

        gl.uniform1i(shader.uniforms.pass, this.model.shaderMode != undefined? this.model.shaderMode : shaderConstants.MODE_TEXTURE);
        gl.activeTexture(gl.TEXTURE0);
        gl.uniform1i(shader.uniforms.screen, 0);
        gl.bindTexture(gl.TEXTURE_2D, this.texture);
        const buff = this.model.buffers.buffer;

        if (isDirty)
            gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, this.width, this.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, buff);
        this.renderer.render(shader, this.renderer.quad);


    }
}
 