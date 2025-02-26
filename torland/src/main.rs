use glutin_window::GlutinWindow as Window;
use graphics::rectangle::Border;
use graphics::{clear, Context, DrawState, Image, Rectangle};
use opengl_graphics::{CreateTexture, Format, GlGraphics, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{Button, Key, MouseButton, MouseCursorEvent, PressEvent, UpdateEvent};
use torland::util::{self, get_coler_by_id, get_coler_name_by_id, COLERS_CNT};

const WINDOW_H: f64 = 400.0;
const WINDOW_W: f64 = 400.0;
const WORLD_H: usize = 20;
const WORLD_W: usize = 20;

const Y_STEP: f64 = WINDOW_H as f64 / WORLD_H as f64;
const X_STEP: f64 = WINDOW_W as f64 / WORLD_W as f64;

fn main() {
    let mut window: Window = WindowSettings::new("", [WINDOW_H, WINDOW_W])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let config = r#"
    {
        "sun_max_lvl": 10,
        "mineral_max_lvl": 10,
        "height": 20,
        "width": 20,
        "word_type": "Clustered",
        "cluster_cnt": 100,
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
            "mutation_ver": 0.01,
            "energy_per_sun_free_boost": 10,
            "energy_per_sun_bro_boost": 5,
            "energy_per_sun_oth_boost": -2
        }
    }
    "#;
    let mut world = util::make_world(&config)
        .map_err(|e| eprintln!("Failed to create world: {e}"))
        .unwrap();

    let mut coler_id = 0;
    let mut coler = get_coler_by_id(coler_id);
    window.window.set_title(get_coler_name_by_id(coler_id));

    let mut pause = true;
    let mut by_step = true;
    let mut cursor_pos = [0.0 as f64; 2];

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    let mut background_texture_bytes = [0u8; WORLD_H*WORLD_W*4/*rgba - 4 bytes*/];
    let i = world.get_info();
    world.foreach_cell(|x, y, cell| {
        let color = [
            (cell.sun * 255 / i.max_sun) as u8,
            (cell.sun * 255 / i.max_sun) as u8,
            (cell.mineral * 255 / i.max_mineral) as u8,
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
                let world_info = world.get_info();
                world.foreach_bot(|x, y, b| {
                    let rect = [X_STEP * x as f64, Y_STEP * y as f64, X_STEP, Y_STEP];
                    let (cr, cg, cb) = coler(&world_info, &b.get_info());
                    Rectangle::new([cr as f32 / 255.0, cg as f32 / 255.0, cb as f32 / 255.0, 1.0])
                        .draw(rect, &Default::default(), c.transform, g);
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

            if let Button::Keyboard(Key::Space) = args {
                coler_id = (coler_id + 1) % COLERS_CNT;
                coler = get_coler_by_id(coler_id);
                window.window.set_title(get_coler_name_by_id(coler_id));
            }

            if let Button::Mouse(MouseButton::Left) = args {
                let pos = (
                    (cursor_pos[0] / X_STEP) as usize,
                    (cursor_pos[1] / Y_STEP) as usize,
                );
                world
                    .spawn(
                        pos.into(),
                        "YWJTCDWCGAGEKQ2TUAMWBAANZDBAUC3CQWSSWLAIOEBY5QRJHBQY7UKEJVKOWK2O4UXLKZFZ2X773UMTTOH6U2Q74YWMZA3LUP5IV6UX5AK5C3MYQ3UCV4L5ZTPZCZYL32JHRZ5OK6IBCS53LYZ7SG3YL64N5AVXMBMHZ3SVZCSYKXCLSKK6FMQCVYRXRSYJXH6HTJYM25M4A5IZWEXTZE73J6VQC3RVMKP3ODHXLXAF2Z2YBTSNNQT5JK47Z7L3MS4J4AVOC54BXTPP6MG54H7DRX5L6SHLFU",
                        // "MNSIAABREADAA",
                        // "PWFUWCQAEAEAL3NXNJL5D7IP2ZQXUZECFZ2GAEHEJUTWNQYBFM6I2777HQLFSMSMIET625F2UI3BO"
                    )
                    .ok();
            }

            if let Button::Mouse(MouseButton::Right) = args {
                let pos = (
                    (cursor_pos[0] / X_STEP) as usize,
                    (cursor_pos[1] / Y_STEP) as usize,
                );
                if let Ok(i) = world.get_bot_info(pos.into()) {
                    eprintln!("{}", i);
                    eprintln!(
                        "code: \n=========================\n{}=========================",
                        botc::compiler::decompile(Vec::clone(&i.genom)).iter().fold(
                            String::new(),
                            |mut acc, cmd| {
                                acc.push_str(format!("{}", cmd).as_str());
                                acc.push_str("\n");
                                acc
                            }
                        )
                    );
                } else {
                    eprintln!("No bot");
                }
            }
        }
    }
}

fn draw_cursor(cursor_pos: &[f64; 2], c: Context, g: &mut GlGraphics) {
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
