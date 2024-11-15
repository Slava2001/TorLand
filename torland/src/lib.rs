pub mod vec2;
pub mod voronoi;
pub mod world;

#[cfg(target_arch = "wasm32")]
use {
    rand::{thread_rng, Rng},
    std::cell::Ref,
    std::usize,
    voronoi::Voronoi,
    wasm_bindgen::{prelude::*, Clamped},
    web_sys::{CanvasRenderingContext2d, ImageData},
    world::{Bot, Rules, World, WorldConfig},
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct WorldWraper {
    world: world::World,
    w: usize,
    h: usize,
    sun_max_lvl: usize,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WorldWraper {
    pub fn new(h: usize, w: usize, cnt: usize) -> Self {
        const SUN_MAX_LVL: usize = 10;

        let mut colors = Vec::<vec2::Vec2u>::new();
        for _ in 0..cnt {
            let sun = thread_rng().gen_range(0..SUN_MAX_LVL) as usize;
            colors.push((sun, cnt - sun + thread_rng().gen_range(0..2)).into());
        }
        colors.push((0usize, 0).into());
        let voron = Voronoi::new(&mut thread_rng(), h, w, cnt);

        Self {
            world: World::new(WorldConfig {
                h,
                w,
                rules: Rules::default(),
                sun: |x, y| colors[voron.get(x, y)].x,
                mineral: |x, y| colors[voron.get(x, y)].y,
            }),
            h,
            w,
            sun_max_lvl: SUN_MAX_LVL,
        }
    }

    pub fn update(&mut self) {
        self.world.update().ok();
    }

    fn get_color(seed: usize) -> String {
        const M: usize = 1 << 31;
        const A: usize = 1103515245;
        const C: usize = 12345;
        let color = (A * seed + C) % M;
        format!(
            "rgba({}, {}, {}, 255)",
            (color % 0xff) as u8,
            (color >> 8 % 0xff) as u8,
            (color >> 16 % 0xff) as u8
        )
    }

    pub fn draw(&mut self, ctx: &CanvasRenderingContext2d, color_mod: usize) {
        let color: fn(Ref<Bot>) -> String = match color_mod {
            0 => |c| Self::get_color(c.get_colony()),
            1 => |_| "rgba(255, 0, 0, 255)".into(),
            2 => |_| "rgba(0, 255, 0, 255)".into(),
            3 => |_| "rgba(0, 0, 255, 255)".into(),
            _ => |_| "rgba(0, 255, 0, 255)".into(),
        };
        self.world.foreach_bot(|x, y, c| {
            ctx.set_fill_style_str(&color(c));
            ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
        });
    }

    pub fn draw_bg(&mut self, ctx: &CanvasRenderingContext2d) {
        let mut img_buff = vec![0; self.w * self.h * 4];
        self.world.foreach_cell(|x, y, cell| {
            let color = [
                (cell.sun * 255 / self.sun_max_lvl) as u8,
                (cell.sun * 255 / self.sun_max_lvl) as u8,
                (cell.mineral * 255 / self.sun_max_lvl) as u8,
                255,
            ];
            img_buff[(y * self.w + x) * 4..(y * self.w + x + 1) * 4].copy_from_slice(&color);
        });

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&img_buff[..(self.h * self.w * 4)]),
            self.w as u32,
            self.h as u32,
        )
        .expect("should create ImageData from array");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("should write array to context");
    }

    pub fn spawn(&mut self, x: usize, y: usize, gen_b32: &str) {
        self.world.spawn((x, y).into(), gen_b32).ok();
    }
}
