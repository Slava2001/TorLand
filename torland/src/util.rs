use crate::voronoi::Voronoi;
use crate::world::{bot, Rules, World, WorldConfig};
use crate::{vec2, world};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::usize;

#[derive(Deserialize)]
struct Config {
    sun_max_lvl: usize,
    mineral_max_lvl: usize,
    height: usize,
    width: usize,
    word_type: WorldType,
    rules: Rules,
    cluster_cnt: Option<usize>
}
#[derive(Deserialize, Serialize)]
enum WorldType {
    Uniform,
    Linear,
    Clustered,
}

pub fn make_world(cfg: &str) -> Result<World, String> {
    let cfg: Config =
        serde_json::from_str(cfg).map_err(|e| format!("Failed to parse config: {e}"))?;

    match cfg.word_type {
        WorldType::Uniform => Ok(World::new(WorldConfig {
            h: cfg.height,
            w: cfg.width,
            rules: cfg.rules,
            sun: |_, _| cfg.sun_max_lvl,
            mineral: |_, _| cfg.mineral_max_lvl,
        })),
        WorldType::Linear => Ok(World::new(WorldConfig {
            h: cfg.height,
            w: cfg.width,
            rules: cfg.rules,
            sun: |_, y| (cfg.height - y) * cfg.sun_max_lvl / cfg.height,
            mineral: |_, y| y * cfg.sun_max_lvl / cfg.height,
        })),
        WorldType::Clustered => {
            let cluster_cnt = cfg.cluster_cnt.unwrap_or(1);
            let mut cluster_info = Vec::<vec2::Vec2u>::new();
            for _ in 0..cluster_cnt {
                let sun = thread_rng().gen_range(0..=cfg.sun_max_lvl);
                let mineral = thread_rng().gen_range(0..=cfg.mineral_max_lvl);
                cluster_info.push((sun, mineral).into());
            }
            cluster_info.push((0usize, 0).into());
            let voron = Voronoi::new(&mut thread_rng(), cfg.height, cfg.width, cluster_cnt);
            Ok(World::new(WorldConfig {
                h: cfg.height,
                w: cfg.width,
                rules: cfg.rules,
                sun: |x, y| cluster_info[voron.get(x, y)].x,
                mineral: |x, y| cluster_info[voron.get(x, y)].y,
            }))
        }
    }
}

fn get_color_by_id(seed: usize) -> (u8, u8, u8) {
    const M: usize = 1 << 31;
    const A: usize = 1103515245;
    const C: usize = 12345;
    let color = (A * seed + C) % M;
    (
        (color % 0xff) as u8,
        (color >> 8 % 0xff) as u8,
        (color >> 16 % 0xff) as u8,
    )
}

fn get_color_grad(x: f32) -> (u8, u8, u8) {
    const COLOR_1: (u8, u8, u8) = (0, 0, 0);
    const COLOR_2: (u8, u8, u8) = (255, 255, 255);
    (
        (COLOR_1.0 as f32 + (COLOR_2.0 as f32 - COLOR_1.0 as f32) * x) as u8,
        (COLOR_1.1 as f32 + (COLOR_2.1 as f32 - COLOR_1.1 as f32) * x) as u8,
        (COLOR_1.2 as f32 + (COLOR_2.2 as f32 - COLOR_1.2 as f32) * x) as u8,
    )
}

pub const COLERS_CNT: usize = 7;
pub fn get_coler_by_id(id: usize) -> fn(&world::Info, &bot::Info) -> (u8, u8, u8) {
    match id % COLERS_CNT {
        0 => |_, b| get_color_by_id(b.colony_id),
        1 => |_, b| get_color_by_id(b.genom_id),
        2 => |w, b| get_color_grad((b.age - w.min_age) as f32 / (w.max_age - w.min_age) as f32),
        3 => |w, b| {
            get_color_grad((b.energy - w.min_energy) as f32 / (w.max_energy - w.min_energy) as f32)
        },
        4 => |_, _| (255, 0, 0),
        5 => |_, _| (0, 255, 0),
        6 => |_, _| (0, 0, 255),
        _ => unreachable!(),
    }
}
pub fn get_coler_name_by_id(id: usize) -> &'static str {
    match id % COLERS_CNT {
        0 => "by colony",
        1 => "by genom",
        2 => "by age",
        3 => "by energy",
        4 => "reg",
        5 => "green",
        6 => "blue",
        _ => unreachable!(),
    }
}
