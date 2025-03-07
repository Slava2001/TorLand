import init, { WorldWraper } from '/torland/torland.js';
init();

const canvas = document.getElementById("canvas");
const bot_code = document.getElementById("bot_code");
const fps = document.getElementById("fps");
const coler = document.getElementById(colerId);
const lmb_action = document.getElementById(actionId);
const start_btn = document.getElementById("start_btn");

let background;
let world;
let timer;
let world_size_x;
let world_size_y;

function render_word() {
    canvas.getContext("2d").drawImage(background, 0, 0);
    world.draw(canvas.getContext("2d"), coler.value);
}
coler.onchange = () => {
    if (world) {
        render_word();
    }
}

function update() {
    world.update();
    render_word();
    timer = setTimeout(update, 1000 / fps.value);
}

function get_config() {

    let input = (name, default_value) => {
        let input = document.getElementById(name)
        if (input.value) {
            return input.value
        }
        return default_value
    }

    let cfg = `
    {
    "sun_max_lvl": ${input("sun_max_lvl", 10)},
    "mineral_max_lvl": ${input("mineral_max_lvl", 10)},
    "height": ${input("height", 200)},
    "width": ${input("width", 200)},
    "word_type": ${input("word_type", '"Clustered"')},
    "cluster_cnt": ${input("cluster_cnt", 20)},
    "rules": {
        "max_commands_per_cycle":  ${input("max_commands_per_cycle", 10)},
        "energy_for_split":  ${input("energy_for_split", 1000)},
        "energy_per_sun":  ${input("energy_per_sun", 10)},
        "energy_per_mineral":  ${input("energy_per_mineral", 10)},
        "energy_per_step":  ${input("energy_per_step", 50)},
        "age_per_energy_penalty":  ${input("age_per_energy_penalty", 100)},
        "start_energy":  ${input("start_energy", 100)},
        "on_bite_energy_delimiter":  ${input("on_bite_energy_delimiter", 10)},
        "max_energy":  ${input("max_energy", 10000)},
        "max_random_value":  ${input("max_random_value", 10000)},
        "mutation_ver":  ${input("mutation_ver", 0.1)},
        "energy_per_sun_free_boost":  ${input("energy_per_sun_free_boost", 5)},
        "energy_per_sun_bro_boost":  ${input("energy_per_sun_bro_boost", 10)},
        "energy_per_sun_oth_boost":  ${input("energy_per_sun_oth_boost", -2)}
        }
    }
    `

    return cfg
}

function run() {
    let cfg = JSON.parse(get_config());
    world_size_x = cfg["width"];
    world_size_y = cfg["height"];
    try {
        world = WorldWraper.new(get_config());
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
}
document.getElementById("create_word").onclick = run;

function on_click(e) {
    if (!world) {
        return;
    }
    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left) * world_size_x / rect.width);
    const y = Math.floor((e.clientY - rect.top) * world_size_y / rect.height);
    if (lmb_action.value == "place") {
        world.spawn(x, y, bot_code.value);
        render_word();
    } else {
        bot_code.value = world.get_bot(x, y);
    }
}
canvas.onclick = on_click

function on_start_btn_click(e) {
    if (!world) {
        return;
    }
    if (start_btn.innerHTML == startLabel) {
        start_btn.innerHTML = stopLabel;
        clearTimeout(timer);
        update();
    } else {
        start_btn.innerHTML = startLabel;
        clearTimeout(timer);
    }
}
start_btn.onclick = on_start_btn_click
