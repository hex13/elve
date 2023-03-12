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
        return this;
    }
    createTexture(width, height) {
        const gl = this.gl;
        const texture = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
        return texture;
    }
    createBuffer(data) {
        const gl = this.gl;
        const buffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
        gl.bufferData(gl.ARRAY_BUFFER, data, gl.DYNAMIC_DRAW);
        return buffer;
    }
    createRenderable(data, componentsPerVertex) {
        return {
            buffer: this.createBuffer(data),
            componentsPerVertex,
            count: data.length / componentsPerVertex,
        };
    }
    render(shader, renderable) {
        const gl = this.gl;
        gl.bindBuffer(gl.ARRAY_BUFFER, renderable.buffer);
        gl.vertexAttribPointer(shader.attributes.aPosition, renderable.componentsPerVertex, gl.FLOAT, false, 0, 0);
        gl.enableVertexAttribArray(shader.attributes.aPosition);
        gl.drawArrays(gl.TRIANGLES, 0, renderable.count);
    }

}