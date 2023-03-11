import {EventKind} from './pkg/elve.js';

export function addListeners(canvas, mainApp, drawingEditor) {
    const createEventHandler = (kind) =>  e => {
        const bounds = e.target.getBoundingClientRect();
        const canvasX = e.clientX - bounds.left;
        const canvasY = e.clientY - bounds.top;

        const scaleX = bounds.width / drawingEditor.width;
        const scaleY = bounds.height / drawingEditor.height;
        mainApp.dispatch(kind, canvasX / scaleX, canvasY / scaleY);
    };

    const handlers = {
        pointerDown: createEventHandler(EventKind.PointerDown),
        pointerMove: createEventHandler(EventKind.PointerMove),
        pointerUp: createEventHandler(EventKind.PointerUp),
    };
    canvas.addEventListener('pointerdown', handlers.pointerDown);
    canvas.addEventListener('pointermove', handlers.pointerMove);
    canvas.addEventListener('pointerup', handlers.pointerUp);
    canvas.addEventListener('touchstart', e => {
        e.preventDefault();
    });
}
