'use strict';


//=================================================================================================

const CURSOR = {
    buttons: 0,
    prevCoordX: null,
    prevCoordY: null,
    coordX: null,
    coordY: null,
    x: null,
    y: null,
};

function mouse(e) {
    CURSOR.buttons = e.buttons;
    CURSOR.prevCoordX = CURSOR.coordX;
    CURSOR.prevCoordY = CURSOR.coordY;
    CURSOR.coordX = e.x;
    CURSOR.coordY = e.y;
    CURSOR.x = (e.x - CANVAS.offsetLeft) / CANVAS.offsetWidth;
    CURSOR.y = (e.y - CANVAS.offsetTop) / CANVAS.offsetHeight;
}

CANVAS.addEventListener('mousedown', mouse);
CANVAS.addEventListener('mouseup', mouse);
CANVAS.addEventListener('mousemove', mouse);

//=================================================================================================

const ARROW_UP = "ARROWUP";
const ARROW_DOWN = "ARROWDOWN";
const ARROW_LEFT = "ARROWLEFT";
const ARROW_RIGHT = "ARROWRIGHT";

const KEYS = {};

function key_pressed(e) {
    KEYS[e.key.toUpperCase()] = true;
}
function key_released(e) {
    KEYS[e.key.toUpperCase()] = false;
}

document.addEventListener("keydown", key_pressed);
document.addEventListener("keyup", key_released);
