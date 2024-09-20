//while(true){
let msg = JSON.parse(await dioxus.recv());

const canvas=document.getElementById("tetris");
canvas.height=600;
canvas.width=600;
const ctx = canvas.getContext('2d'); 
let field=msg["field"];
const hold=msg["hold"];
const nexts=msg["nexts"];
const limit_view=20;
const blockSize = 20;
const field_distance=200;//どれくらいfieldを右にずらすか(px)
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

field=field.slice(field.length-limit_view,field.length);
function draw_field() {
    for (let row = 0; row < field.length; row++) {
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
            ctx.fillRect(field_distance+col * blockSize, row * blockSize, blockSize, blockSize);//塗りつぶし
            ctx.strokeRect(field_distance+col * blockSize, row * blockSize, blockSize, blockSize);//枠線
        }
    }
}
draw_field();
//*}