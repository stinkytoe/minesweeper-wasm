import * as wasm from "minesweeper-wasm";

//wasm.greet();

const CELL_SIZE = 32;

const tile_images = new Image()
tile_images.src = 'res/minesweeper-tiles.png'

let game = wasm.WasmGame.new();
game.stub();

function process_remaining_mines_section() {
    document.getElementById("remaining-mines").textContent = "Mines left: " + game.get_remaining_mines();
}

function process_gamestate_section() {
    document.getElementById("gamestate-section").textContent = game.get_game_state();
}

let leftdown = false;
let middown = false;
let rightdown = false;
function process_board_section() {

    function process_mouse_movement(x, y) {
        if (leftdown && !rightdown) 
            game.mark(x,y);
        if (leftdown && rightdown)
            game.mark_block(x,y);
    }
    
    function ev_mousedown(ev) {
        let x = Math.trunc(ev.offsetX/CELL_SIZE);
        let y = Math.trunc(ev.offsetY/CELL_SIZE);
        if (ev.button == 0) 
            leftdown = true;
        if (ev.button == 1)
             middown = true;
        if (ev.button == 2) {
            rightdown = true;
            if (!leftdown) 
                game.toggle_flag(x,y);
        }
    
        process_mouse_movement(x,y);
    
        //document.getElementById("message-section").textContent = "l:"+leftdown+" m:"+middown+" r:"+rightdown;
    }

    function ev_mouseup(ev) {
        if (ev.button == 0) 
            leftdown = false;
        if (ev.button == 1)
             middown = false;
        if (ev.button == 2)
            rightdown = false;
        
        game.dig_marked();
    
        //document.getElementById("message-section").textContent = "l:"+leftdown+" m:"+middown+" r:"+rightdown;
    }

    function ev_mousemove(ev) {
        let x = Math.trunc(ev.offsetX/CELL_SIZE);
        let y = Math.trunc(ev.offsetY/CELL_SIZE);
    
        process_mouse_movement(x,y);
    }

    let canvas = document.getElementById("minesweeper-field");
    if (canvas == null) {
        canvas = document.createElement('canvas');
        canvas.id = "minesweeper-field";
        canvas.width = game.get_rows() * CELL_SIZE;
        canvas.height = game.get_cols() * CELL_SIZE;
        // canvas.addEventListener('mousedown', ev_mousedown);
        // canvas.addEventListener('contextmenu', function() {return false;});
        canvas.onmousedown = ev_mousedown;
        canvas.onmouseup = ev_mouseup;
        canvas.onmousemove = ev_mousemove;
        canvas.oncontextmenu = function(){return false;}; // to disable context menu in game canvas
        //document.body.appendChild(canvas);
        document.getElementById('board-section').appendChild(canvas);
    }
    const ctx = canvas.getContext('2d'); 

    const tile_promises = [];

    for (let i=0; i<4; i++) {
        for (let j=0; j<4; j++) {
            tile_promises.push(
                createImageBitmap(
                    tile_images, 
                    j*CELL_SIZE, i*CELL_SIZE, CELL_SIZE, CELL_SIZE));
        }
    }

    Promise
        .all(tile_promises)
        .then(function(tiles) {
            for (let i=0; i<game.get_rows(); i++) {
                for (let j=0; j<game.get_cols(); j++) {
                    ctx.drawImage(tiles[game.get_cell_code(i,j)], i*CELL_SIZE, j*CELL_SIZE);
                }
            }
        });
}

const renderLoop = () => {
    process_remaining_mines_section();

    process_gamestate_section();

    process_board_section();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);