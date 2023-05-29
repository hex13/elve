import {FireworksWebGLRenderable} from './renderables/FireworksWebGLRenderable.ts';
import {DrawingEditorWebGLRenderable} from './renderables/DrawingEditorWebGLRenderable.ts';

export const rendererConstructors = {
    fireworks: FireworksWebGLRenderable,
    drawingEditor: DrawingEditorWebGLRenderable,
    extra: DrawingEditorWebGLRenderable,
};


export function createModelFactories(app, appConfig) {
    const { width, height } = appConfig.size;
    return {
        fireworks: () => app.add_fireworks_model(),
        drawingEditor: () => app.add_drawing_editor_model(width, height),
        extra: () => app.add_extra_model(width, height),
    };
};
