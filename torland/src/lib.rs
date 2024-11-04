pub mod world;

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
    pub fn new(s: usize) -> Self {
        Self {
            world: World::new(world::WorldConfig {
                h: s,
                w: s,
                thread_cnt: 1,
                code: "".into(),
            }),
            size: std::cmp::min(s, 100),
        }
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let buf = &mut [0; 100 * 100 * 4];
        self.world.for_each_cell(|x, y, c| {
            let color = match c {
                world::Cell::None => [0; 4],
                world::Cell::Bot(_, _bot) => [0, 255, 0, 255],
            };
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
