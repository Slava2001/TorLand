import init, { WorldWraper } from '/torland/torland.js';
init();

const canvas = document.getElementById("canvas");
const bot_code = document.getElementById("bot_code");
const fps = document.getElementById("fps");

const start_btn = document.getElementById("start_btn");

let background;
let world;
let timer;
let world_size_x;
let world_size_y;

function render_word() {
    canvas.getContext("2d").drawImage(background, 0, 0);

    const coler = document.getElementById(colerId);
    world.draw(canvas.getContext("2d"), coler.value);
}

const coler_ru = document.getElementById("coler_ru");
coler_ru.onchange = () => {
    if (world) {
        render_word();
    }
}

const coler_en = document.getElementById("coler_en");
coler_en.onchange = () => {
    if (world) {
        render_word();
    }
}

function update() {
    world.update();
    render_word();
    timer = setTimeout(update, 1000 / fps.value);
}

function run() {
    const context = canvas.getContext('2d');
    context.clearRect(0, 0, canvas.width, canvas.height);

    let cfg = JSON.parse(get_config());

    world_size_x = cfg["width"];
    world_size_y = cfg["height"];
    try {
        world = WorldWraper.new(JSON.stringify(cfg));
    } catch (e) {
        alert(e);
        return
    }
    canvas.height = world_size_y;
    canvas.width = world_size_x;
    world.draw_bg(canvas.getContext("2d"));
    background = new Image();
    background.src = canvas.toDataURL();
    clearTimeout(timer);
    start_btn.innerHTML = startLabel;
    start_btn.classList.remove("stop-button");
    start_btn.classList.remove("disabled-button");
    start_btn.classList.add("start-button");
}
document.getElementById("create_word").onclick = run;

function on_click(e) {
    if (!world) {
        return;
    }
    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left) * world_size_x / rect.width);
    const y = Math.floor((e.clientY - rect.top) * world_size_y / rect.height);

    const lmb_action = document.getElementById(actionId);
    if (lmb_action.value == "place") {
        world.spawn(x, y, bot_code.value);
        render_word();
    } else {
        bot_code.value = world.get_bot(x, y);
        setCookie("bot_code", bot_code.value, 3);
    }
}
canvas.onclick = on_click

function on_start_btn_click(e) {
    if (!world) {
        return;
    }
    if (start_btn.innerHTML == startLabel) {
        start_btn.innerHTML = stopLabel;
        start_btn.classList.add("stop-button");
        start_btn.classList.remove("start-button");

        clearTimeout(timer);
        update();
    } else {
        start_btn.innerHTML = startLabel;
        start_btn.classList.add("start-button");
        start_btn.classList.remove("stop-button");
        clearTimeout(timer);
    }
}
start_btn.onclick = on_start_btn_click
