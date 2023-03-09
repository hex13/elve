import {EventKind} from './pkg/elve.js';

export function addListeners(canvas, mainApp, drawingEditor) {
    canvas.addEventListener('pointerdown', e => {
        const bounds = e.target.getBoundingClientRect();
        const canvasX = e.clientX - bounds.left;
        const canvasY = e.clientY - bounds.top;

        const scaleX = bounds.width / drawingEditor.width;
        const scaleY = bounds.height / drawingEditor.height;
        mainApp.dispatch(EventKind.PointerDown, canvasX / scaleX, canvasY / scaleY);
    });

    canvas.addEventListener('touchstart', e => {
        e.preventDefault();
    });

    canvas.addEventListener('pointermove', e => {
        const bounds = e.target.getBoundingClientRect();
        const canvasX = e.clientX - bounds.left;
        const canvasY = e.clientY - bounds.top;

        const scaleX = bounds.width / drawingEditor.width;
        const scaleY = bounds.height / drawingEditor.height;
        mainApp.dispatch(EventKind.PointerMove, canvasX / scaleX, canvasY / scaleY);
    });

    canvas.addEventListener('pointerup', e => {
        const bounds = e.target.getBoundingClientRect();
        const canvasX = e.clientX - bounds.left;
        const canvasY = e.clientY - bounds.top;

        const scaleX = bounds.width / drawingEditor.width;
        const scaleY = bounds.height / drawingEditor.height;
        mainApp.dispatch(EventKind.PointerUp, canvasX / scaleX, canvasY / scaleY);
    });
}
