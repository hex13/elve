import {FireworksWebGLRenderable} from './renderables/FireworksWebGLRenderable.ts';
import {DrawingEditorWebGLRenderable} from './renderables/DrawingEditorWebGLRenderable.ts';

export const rendererConstructors = {
    fireworks: FireworksWebGLRenderable,
    drawingEditor: DrawingEditorWebGLRenderable,
    extra: DrawingEditorWebGLRenderable,
};
