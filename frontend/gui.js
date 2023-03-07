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

function FireworksControlls({onChangeAutoexplosions, onChangeController}) {
    return <div>
        <label>auto explosions: <input onChange={onChangeAutoexplosions} type="checkbox" /></label>
        <button onClick={() => onChangeController(0)}>fireworks</button>
        <button onClick={() => onChangeController(1)}>rectangles</button>
        <button onClick={() => onChangeController(2)}>drawing</button>
    </div>
}

export function createGUI(el, { changeLayers, changeAutoexplosions, changeController }) {
    const root = ReactDOM.createRoot(el)
    let layers = [
        {id: 0, name: 'first'},
        {id: 1, name: 'second'},
        {id: 2, name: 'third'},
    ];
    function render() {
        const tree = <>
            <Layers layers={layers} onSwap={(aId, bId) => {
                const a = layers.find(layer => layer.id == aId);
                const b = layers.find(layer => layer.id == bId);
                layers = layers.map(layer => {
                    if (layer.id == aId) return b;
                    if (layer.id == bId) return a;
                    return layer;
                });
                changeLayers(layers);
                render();
            }} />
            <FireworksControlls onChangeAutoexplosions={changeAutoexplosions} onChangeController={changeController}/>
        </>;
        root.render(tree);
    }
    render();
}
