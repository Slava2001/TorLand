use glutin_window::GlutinWindow as Window;
use graphics::rectangle::Border;
use graphics::{clear, Context, DrawState, Image, Rectangle};
use opengl_graphics::{CreateTexture, Format, GlGraphics, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{Button, Key, MouseButton, MouseCursorEvent, PressEvent, UpdateEvent};
use rand::{thread_rng, Rng};
use torland::vec2::Vec2u;
use torland::voronoi::Voronoi;
use torland::world::{Rules, World, WorldConfig};

const WINDOW_H: f64 = 800.0;
const WINDOW_W: f64 = 800.0;
const WORLD_H: usize = 400;
const WORLD_W: usize = 400;
const CLUSTER_CNT: usize = 50;
const SUN_MAX_LVL: usize = 10;

fn main() {
    let mut window: Window = WindowSettings::new("TorLand", [WINDOW_H, WINDOW_W])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut colors = Vec::<Vec2u>::new();
    for _ in 0..CLUSTER_CNT {
        let sun = thread_rng().gen_range(0..SUN_MAX_LVL) as usize;
        colors.push((sun, SUN_MAX_LVL - sun + thread_rng().gen_range(0..2)).into());
    }
    colors.push((0usize, 0).into());

    let voron = Voronoi::new(&mut thread_rng(), WORLD_H, WORLD_W, CLUSTER_CNT);

    let world_cfg = WorldConfig {
        h: WORLD_H,
        w: WORLD_W,
        rules: Rules::default(),
        sun: |x, y| colors[voron.get(x, y)].x,
        mineral: |x, y| colors[voron.get(x, y)].y,
    };
    let mut world = World::new(world_cfg);

    let mut pause = true;
    let mut by_step = true;
    let mut cursor_pos = [0.0 as f64; 2];

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    let mut background_texture_bytes = [0u8; WORLD_H*WORLD_W*4/*rgba - 4 bytes*/];
    world.foreach_cell(|x, y, cell| {
        let color = [
            (cell.sun * 255 / SUN_MAX_LVL) as u8,
            (cell.sun * 255 / SUN_MAX_LVL) as u8,
            (cell.mineral * 255 / SUN_MAX_LVL) as u8,
            255u8,
        ];
        background_texture_bytes[(y * WORLD_W + x) * 4..(y * WORLD_W + x + 1) * 4]
            .copy_from_slice(&color);
    });
    let backgrount_settings = TextureSettings::new().filter(opengl_graphics::Filter::Nearest);
    let background_texture = CreateTexture::create(
        &mut (),
        Format::Rgba8,
        &background_texture_bytes,
        [WORLD_W as u32, WORLD_H as u32],
        &backgrount_settings,
    )
    .unwrap();
    let background_img = Image::new()
        .src_rect([0.0, 0.0, WORLD_W as f64, WORLD_H as f64])
        .rect([0.0, 0.0, WINDOW_W, WINDOW_H]);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);
                background_img.draw(&background_texture, &DrawState::default(), c.transform, g);
                const Y_STEP: f64 = WINDOW_H as f64 / WORLD_H as f64;
                const X_STEP: f64 = WINDOW_W as f64 / WORLD_W as f64;
                world.foreach_bot(|x, y, _| {
                    let rect = [X_STEP * x as f64, Y_STEP * y as f64, X_STEP, Y_STEP];
                    let color = [0.0, 0.0, 0.0, 1.0];
                    Rectangle::new(color).draw(rect, &Default::default(), c.transform, g);
                });
                draw_cursor(&cursor_pos, c, g);
            });
        }

        if let Some(_) = e.update_args() {
            if !pause {
                world.update().ok();
                if by_step {
                    pause = true;
                }
            }
        }

        if let Some(args) = e.mouse_cursor_args() {
            cursor_pos = args;
        }

        if let Some(args) = e.press_args() {
            if let Button::Keyboard(Key::Tab) = args {
                pause = !pause;
            }

            if let Button::Keyboard(Key::S) = args {
                by_step = !by_step;
            }

            if let Button::Keyboard(Key::I) = args {
                eprintln!("pause: {pause}");
                eprintln!("by_step: {by_step}");
            }

            if let Button::Mouse(MouseButton::Left) = args {
                const Y_STEP: f64 = WINDOW_H as f64 / WORLD_H as f64;
                const X_STEP: f64 = WINDOW_W as f64 / WORLD_W as f64;
                world
                    .spawn(
                        (
                            (cursor_pos[0] / X_STEP) as usize,
                            (cursor_pos[1] / Y_STEP) as usize,
                        )
                            .into(),
                        "RXF4CCIAEAGEHUJAPILEJBBM4D7AGOMMAWC5EQ2LH66EWIA3F45JNMYPKHCWT4N7INRWBQWGQTZF2".into()
                    )
                    .ok();
            }
        }
    }
}

fn draw_cursor(cursor_pos: &[f64; 2], c: Context, g: &mut GlGraphics) {
    const Y_STEP: f64 = WINDOW_H as f64 / WORLD_H as f64;
    const X_STEP: f64 = WINDOW_W as f64 / WORLD_W as f64;
    let rect = [
        X_STEP * (cursor_pos[0] / X_STEP).floor(),
        Y_STEP * (cursor_pos[1] / Y_STEP).floor(),
        X_STEP,
        Y_STEP,
    ];
    Rectangle::new([0.0; 4])
        .border(Border {
            color: [0.0, 0.0, 0.0, 1.0],
            radius: 1.0,
        })
        .draw(rect, &Default::default(), c.transform, g);
}
