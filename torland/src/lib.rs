pub mod util;
pub mod vec2;
pub mod voronoi;
pub mod world;

#[cfg(target_arch = "wasm32")]
use {
    std::borrow::Borrow,
    std::usize,
    wasm_bindgen::{prelude::*, Clamped},
    web_sys::{CanvasRenderingContext2d, ImageData},
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct WorldWraper {
    world: world::World,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WorldWraper {
    pub fn new(cfg: &str) -> Result<Self, String> {
        Ok(Self {
            world: util::make_world(cfg)?,
        })
    }

    pub fn update(&mut self) {
        self.world.update().ok();
    }

    pub fn draw(&mut self, ctx: &CanvasRenderingContext2d, color_mod: usize) {
        let coler = util::get_coler_by_id(color_mod);
        let i = self.world.get_info();
        self.world.foreach_bot(|x, y, c| {
            let (r, g, b) = coler(&i, &c.get_info());
            ctx.set_fill_style_str(format!("rgba({}, {}, {}, 255)", r, g, b).as_str());
            ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
        });
    }

    pub fn draw_bg(&mut self, ctx: &CanvasRenderingContext2d) {
        let i = self.world.get_info();
        let mut img_buff = vec![0; i.w * i.h * 4];
        self.world.foreach_cell(|x, y, cell| {
            let color = [
                (cell.sun * 255 / i.max_sun) as u8,
                (cell.sun * 255 / i.max_sun) as u8,
                (cell.mineral * 255 / i.max_mineral) as u8,
                255,
            ];
            img_buff[(y * i.w + x) * 4..(y * i.w + x + 1) * 4].copy_from_slice(&color);
        });

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&img_buff[..(i.h * i.w * 4)]),
            i.w as u32,
            i.h as u32,
        )
        .expect("should create ImageData from array");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("should write array to context");
    }

    pub fn spawn(&mut self, x: usize, y: usize, gen_b32: &str) {
        self.world.spawn((x, y).into(), gen_b32).ok();
    }

    pub fn get_bot(&mut self, x: usize, y: usize) -> String {
        self.world.get_bot_info((x, y).into()).ok().map(|i| {
            botc::code_packer::to_b32(i.genom.borrow()).unwrap_or("Invalid Code".into())
        }).unwrap_or("No Bot".into())
    }
}
