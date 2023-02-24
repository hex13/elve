import React, {useState} from 'react';
import * as ReactDOM from 'react-dom/client';

const layerStyle = {
    padding: 8,
    border: '1px solid rgba(255, 255, 255, 0.2)',
    cursor: 'pointer',
    userSelect: 'none',
};


function Layer({layer, onPointerDown, onPointerUp}) {
    return <div
        onPointerDown={onPointerDown}
        onPointerUp={onPointerUp}
        style={layerStyle}
    >
        {layer.name}
    </div>;
}
function Layers({ layers, onSwap }) {
    const [dragged, setDragged] = useState(null);
    const handlePointerDown = (e, layer) => {
        setDragged(layer);
    }
    const handlePointerUp = (e, layer) => {
        onSwap(dragged.id, layer.id);
    }

    return <div>
        {layers.map(layer => <Layer
            onPointerDown={e => handlePointerDown(e, layer)}
            onPointerUp={e => handlePointerUp(e, layer)}
            key={layer.id}
            layer={layer}
        />)}
    </div>
}

export function createGUI(el) {
    const root = ReactDOM.createRoot(el)
    let layers = [
        {id: 1, name: 'first'},
        {id: 2, name: 'second'},
        {id: 3, name: 'third'},
    ];
    function render() {
        root.render(<Layers layers={layers} onSwap={(aId, bId) => {
            const a = layers.find(layer => layer.id == aId);
            const b = layers.find(layer => layer.id == bId);
            layers = layers.map(layer => {
                if (layer.id == aId) return b;
                if (layer.id == bId) return a;
                return layer;
            });
            render();
        }} />);
    }
    render();
}
