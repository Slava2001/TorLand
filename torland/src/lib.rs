pub mod util;
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
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WorldWraper {
    pub fn new(h: usize, w: usize, cnt: usize) -> Self {
        Self {
            world: make_word(h, w, cnt),
        }
    }

    pub fn update(&mut self) {
        self.world.update().ok();
    }

    pub fn draw(&mut self, ctx: &CanvasRenderingContext2d, color_mod: usize) {
        let coler = util::get_coler_by_id(color_mod);
        self.world.foreach_bot(|x, y, c| {
            let (r, g, b) = coler(c);
            ctx.set_fill_style_str(format("rgba({}, {}, {}, 255)", r, g, b));
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
