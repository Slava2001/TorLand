const default_sim_cfg = `
{
    "sun_max_lvl": 10,
    "mineral_max_lvl": 10,
    "height": 200,
    "width": 200,
    "word_type": "Clustered",
    "cluster_cnt": 20,
    "rules": {
        "max_commands_per_cycle": 10,
        "energy_for_split": 1000,
        "energy_per_sun": 10,
        "energy_per_mineral": 10,
        "energy_per_step": 50,
        "age_per_energy_penalty": 100,
        "start_energy": 100,
        "on_bite_energy_delimiter": 10,
        "max_energy": 10000,
        "max_random_value": 10000,
        "mutation_ver": 0.1,
        "energy_per_sun_free_boost": 5,
        "energy_per_sun_bro_boost": 10,
        "energy_per_sun_oth_boost": -2
    }
}
`

function init_sim_config() {
    set_config(JSON.parse(default_sim_cfg))

    let set = (name) => {
        let input = document.getElementById(name);
        let value = getCookie(name);
        if (value) {
            input.value = value
        }

        input.addEventListener("change", () => {
            setCookie(name, input.value, 3);
        });
    }

    set("bot_code")

    set("sun_max_lvl")
    set("mineral_max_lvl")
    set("height")
    set("width")
    set("word_type")
    set("cluster_cnt")

    set("max_commands_per_cycle")
    set("energy_for_split")
    set("energy_per_sun")
    set("energy_per_mineral")
    set("energy_per_step")
    set("age_per_energy_penalty")
    set("start_energy")
    set("on_bite_energy_delimiter")
    set("max_energy")
    set("max_random_value")
    set("mutation_ver")
    set("energy_per_sun_free_boost")
    set("energy_per_sun_bro_boost")
    set("energy_per_sun_oth_boost")
}

function set_config(cfg) {
    let input = (config, name) => {
        let input = document.getElementById(name)
        input.value = config[name]
    }

    input(cfg, "sun_max_lvl")
    input(cfg, "mineral_max_lvl")
    input(cfg, "height")
    input(cfg, "width")
    input(cfg, "word_type")
    input(cfg, "cluster_cnt")

    input(cfg["rules"], "max_commands_per_cycle")
    input(cfg["rules"], "energy_for_split")
    input(cfg["rules"], "energy_per_sun")
    input(cfg["rules"], "energy_per_mineral")
    input(cfg["rules"], "energy_per_step")
    input(cfg["rules"], "age_per_energy_penalty")
    input(cfg["rules"], "start_energy")
    input(cfg["rules"], "on_bite_energy_delimiter")
    input(cfg["rules"], "max_energy")
    input(cfg["rules"], "max_random_value")
    input(cfg["rules"], "mutation_ver")
    input(cfg["rules"], "energy_per_sun_free_boost")
    input(cfg["rules"], "energy_per_sun_bro_boost")
    input(cfg["rules"], "energy_per_sun_oth_boost")
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
    "word_type": "${input("word_type", "Clustered")}",
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
