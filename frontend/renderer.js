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
        this.gl = gl;
        return this.gl;
    }
}