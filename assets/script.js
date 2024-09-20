//while(true){
let msg = JSON.parse(await dioxus.recv());
const canvas=document.getElementById("tetris");
const ctx = canvas.getContext('2d'); 
const field=msg["field"];
const hold=msg["hold"];
const nexts=msg["nexts"];
const limit_view=20;
const blockSize = 20;
canvas.height=600;
canvas.width=600;
const colors = {
    "MinoZ": "red",
    "MinoS": "green",
    "MinoI": "cyan",
    "MinoJ": "blue",
    "MinoL": "orange",
    "MinoT": "purple",
    "MinoO": "yellow",
    "Ghost": "lightgray",
    "Empty": "white",
};

debugger
function draw() {
    for (let row = field.length-limit_view; row < field.length; row++) {
        for (let col = 0; col < field[row].length; col++) {
            const cell = field[row][col];
            let cell_color=colors[cell];
            if (cell_color==null){
                if (cell.Ghost == null){
                    ctx.fillStyle = colors[cell.MinoInMotion || cell.MinoBlock];
                }else{
                    ctx.fillStyle=colors["Ghost"];
                }
            }else{
                ctx.fillStyle = cell_color;
            }
            //ctx.fillStyle = cell === "Empty" ? "white" : colors[cell.MinoInMotion || cell.Ghost || cell.MinoBlock];
            ctx.fillRect(col * blockSize, row * blockSize, blockSize, blockSize);
            ctx.strokeRect(col * blockSize, row * blockSize, blockSize, blockSize);
        }
    }
}

draw();
//*}