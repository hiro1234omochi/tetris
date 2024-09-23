//while(true){
let msg = JSON.parse(await dioxus.recv());

const canvas = document.getElementById("tetris");
canvas.height = 600;
canvas.width = 600;
const ctx = canvas.getContext("2d");
let field = msg["field"];
const hold = msg["hold"];
const nexts = msg["nexts"];
const limit_view = 20;
const additional_view = 4;
const blockSize = 20;
const field_distance = 200; //どれくらいfieldを右にずらすか(px)
const nexts_distancce = 400; //同上
const next_distance_in_nexts = 50; //どれくらいnextsの中でそれぞれ下にずらすか
const colors = {
    MinoZ: "red",
    MinoS: "green",
    MinoI: "cyan",
    MinoJ: "blue",
    MinoL: "orange",
    MinoT: "purple",
    MinoO: "yellow",
    Ghost: "lightgray",
    Empty: "white",
};

debugger;

field = field.slice(field.length - limit_view - additional_view, field.length);
function draw_field() {
    for (let row = 0; row < field.length; row++) {
        for (let col = 0; col < field[row].length; col++) {
            const cell = field[row][col];

            //ctx.fillStyle = cell === "Empty" ? "white" : colors[cell.MinoInMotion || cell.Ghost || cell.MinoBlock];
            let should_draw = false;
            if (row < additional_view) {
                if (cell != "Empty") {
                    should_draw = true;
                }
            } else {
                should_draw = true;
            }
            if (should_draw) {
                draw_cell(
                    cell,
                    field_distance + col * blockSize,
                    row * blockSize,
                    blockSize
                );
            }
        }
    }
}
function draw_cell(cell_type, x, y, cell_size) {
    let cell_color = colors[cell_type];
    if (cell_color == null) {
        if (cell_type.Ghost == null) {
            ctx.fillStyle =
                colors[cell_type.MinoInMotion || cell_type.MinoBlock];
        } else {
            ctx.fillStyle = colors["Ghost"];
        }
    } else {
        ctx.fillStyle = cell_color;
    }
    ctx.fillRect(x, y, cell_size, cell_size); //塗りつぶし
    ctx.strokeRect(x, y, cell_size, cell_size); //枠線
}
function draw_hold() {
    if (hold == null) {
        for (let row = 0; row < 2; row++) {
            for (let col = 0; col < 4; col++) {
                draw_cell("Empty", col * blockSize, row * blockSize, blockSize);
            }
        }
    } else {
        for (let row = 0; row < hold.length; row++) {
            for (let col = 0; col < hold[row].length; col++) {
                const cell = hold[row][col];
                draw_cell(cell, col * blockSize, row * blockSize, blockSize);
            }
        }
    }
}
function draw_nexts() {
    let vertical = 0;
    for (let i = 0; i < nexts.length; i++) {
        for (let row = 0; row < nexts[i].length; row++) {
            for (let col = 0; col < nexts[i][row].length; col++) {
                const cell = nexts[i][row][col];
                draw_cell(
                    cell,
                    nexts_distancce + col * blockSize,
                    vertical + row * blockSize,
                    blockSize
                );
            }
        }
        vertical += next_distance_in_nexts;
    }
}
draw_field();
draw_hold();
draw_nexts();
//*}
