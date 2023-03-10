import React, {useState} from 'react';

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

export function Layers({ layers, onSwap }) {
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