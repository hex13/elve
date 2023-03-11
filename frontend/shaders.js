export const shaderConstants = {
    MODE_COLOR: 0,
    MODE_CLEAN_WITH_TRAILS: 1,
    MODE_BLOOM: 2,
    MODE_TEXTURE: 3,
};
const shaderDefines = Object.entries(shaderConstants).map((entry) => `#define ${entry[0]} ${entry[1]}`).join('\n');

const shaderSources = {
    VERTEX_SHADER: /*glsl*/`
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
    FRAGMENT_SHADER:  /*glsl*/`
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

export function createProgram(gl) {
    const shaders = Object.fromEntries(Object.entries(shaderSources).map(([k, source]) => {
        const shader = gl.createShader(gl[k]);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);
        return [k.split('_')[0].toLowerCase(), shader];
    }));

    const program = gl.createProgram();
    gl.attachShader(program, shaders.fragment);
    gl.attachShader(program, shaders.vertex);
    gl.linkProgram(program);

    const uniforms = {
        screen: gl.getUniformLocation(program, 'screen'),
        prevScreen: gl.getUniformLocation(program, 'prevScreen'),
        pass: gl.getUniformLocation(program, 'pass'),
    };

    const attributes = {
        aPosition: gl.getAttribLocation(program, 'aPosition'),
        aColor: gl.getAttribLocation(program, 'aColor'),
    };
    return { program, uniforms, attributes };
}

