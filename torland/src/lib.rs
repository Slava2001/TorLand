pub mod world;
mod vec2;

#[cfg(target_arch = "wasm32")]
use {wasm_bindgen::prelude::*, web_sys::CanvasRenderingContext2d, world::World};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct WorldWraper {
    world: world::World,
    size: usize,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WorldWraper {
    pub fn new(sise: usize, bot: &str) -> Self {
        let mut ww = Self {
            world: World::new(world::WorldConfig {
                h: sise,
                w: sise
            }),
            size: std::cmp::min(s, 100),
        };
        ww.world.spawn((s/2, s/2).into(), bot.into()).ok();
        ww
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let buf = &mut [0; 100 * 100 * 4];
        self.world.foreach_bot(|x, y, _| {
            let color = [0, 255, 0, 255];
            buf[(y * self.size + x) * 4..(y * self.size + x) * 4 + 4].copy_from_slice(&color);
        });

        let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&buf[..(self.size * self.size * 4)]),
            self.size as u32,
            self.size as u32,
        )
        .expect("should create ImageData from array");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("should write array to context");
    }
}
