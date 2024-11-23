use crate::voronoi::Voronoi;
use crate::world::{bot, Rules, World, WorldConfig};
use crate::{vec2, world};
use rand::{thread_rng, Rng};
use std::usize;

pub fn make_word(h: usize, w: usize, cnt: usize) -> World {
    const SUN_MAX_LVL: usize = 10;

    let mut cluster_info = Vec::<vec2::Vec2u>::new();
    for _ in 0..cnt {
        let sun = thread_rng().gen_range(0..SUN_MAX_LVL) as usize;
        cluster_info.push((sun, SUN_MAX_LVL - sun + thread_rng().gen_range(0..2)).into());
    }
    cluster_info.push((0usize, 0).into());
    let voron = Voronoi::new(&mut thread_rng(), h, w, cnt);

    World::new(WorldConfig {
        h,
        w,
        rules: Rules::default(),
        sun: |x, y| cluster_info[voron.get(x, y)].x,
        mineral: |x, y| cluster_info[voron.get(x, y)].y,
    })
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

pub const COLERS_CNT: usize = 6;
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
