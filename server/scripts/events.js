'use strict';


const CURSOR = {
    prevCoordX: null,
    prevCoordY: null,
    coordX: null,
    coordY: null,
    x: null,
    y: null,
    down: false,
    moved: false
};

CANVAS.addEventListener('mousedown', e => {
    CURSOR.down = true;
});

CANVAS.addEventListener('mouseup', e => {
    CURSOR.down = false;
});

CANVAS.addEventListener('mousemove', e => {
    CURSOR.moved = true;
    CURSOR.prevCoordX = CURSOR.coordX;
    CURSOR.prevCoordY = CURSOR.coordY;
    CURSOR.coordX = e.x;
    CURSOR.coordY = e.y;
    CURSOR.x = e.x / CANVAS.offsetWidth;
    CURSOR.y = 1 - e.y / CANVAS.offsetHeight;
});
