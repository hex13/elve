import {EventKind} from './pkg/elve.js';

export function createEventHandlers(canvas, mainApp, drawingEditor) {
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
    return {
        ...handlers,
        changeAutoexplosions() {
            // TODO don't hardcode idx
            const controllerIdx = 0;
            mainApp.dispatch_to(controllerIdx, EventKind.TogglePlay, 0, 0);
        },
        changeController(idx) {
            mainApp.set_controller(idx);
        },
        changeColor(e) {
            const color = parseInt(e.target.value.slice(1), 16);
            // TODO don't hardcode idx
            const controllerIdx = 2;
            mainApp.dispatch_to(controllerIdx, EventKind.ChangeColor, color, 0);
        },
    };
}
