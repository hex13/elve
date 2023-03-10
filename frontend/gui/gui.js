import React, {useState} from 'react';
import * as ReactDOM from 'react-dom/client';
import {Layers} from './layers';


function FireworksControlls({onChangeAutoexplosions, onChangeController, onColorChange}) {
    return <div>
        <label>auto explosions: <input onChange={onChangeAutoexplosions} type="checkbox" /></label>
        <button onClick={() => onChangeController(0)}>fireworks</button>
        <button onClick={() => onChangeController(1)}>rectangles</button>
        <button onClick={() => onChangeController(2)}>drawing</button>
        <input onChange={onColorChange} type="color" />
    </div>
}

export function createGUI(el, { changeLayers, changeAutoexplosions, changeController, changeColor }) {
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
            <FireworksControlls onChangeAutoexplosions={changeAutoexplosions} onChangeController={changeController} onColorChange={changeColor}/>
        </>;
        root.render(tree);
    }
    render();
}
