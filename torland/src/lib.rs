pub mod world;

#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen::prelude::*,
    web_sys::CanvasRenderingContext2d,
    world::World
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct WorldWraper {
    world: world::World
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WorldWraper {
    pub fn new() -> Self {
        web_sys::console::log_1(&"start...".into());
        Self {
            world: World::new(world::WorldConfig { h: 10, w: 10, thread_cnt: 1 }),
        }
    }

    pub fn update(&mut self) {
        web_sys::console::log_1(&"Update...".into());
        self.world.update();
        web_sys::console::log_1(&"UpdateOK...".into());
    }
    
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        web_sys::console::log_1(&"draw...".into());
        
        let buf = &mut [0; 10*10*4];
        self.world.for_each_cell(|x, y, c| {
            let color = match c {
                world::Cell::None => [0; 4],
                world::Cell::Bot(_, _bot) => [0, 255, 0, 255],
            };
            buf[(y * 10 + x)*4..(y * 10 + x)*4 + 4].copy_from_slice(&color);
        });

        let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(buf),
            10,
            10,
        )
        .expect("should create ImageData from array");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("should write array to context");
    }
}