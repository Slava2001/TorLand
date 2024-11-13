use glutin_window::GlutinWindow as Window;
use graphics::rectangle::Border;
use graphics::{clear, Context, Rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{Button, Key, MouseButton, MouseCursorEvent, PressEvent, UpdateEvent};
use torland::world::{Rules, World, WorldConfig};

const WINDOW_H: f64 = 400.0;
const WINDOW_W: f64 = 400.0;
const WORLD_H: usize = 200;
const WORLD_W: usize = 200;

fn main() {
    let mut window: Window = WindowSettings::new("TorLand", [WINDOW_H, WINDOW_W])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let world_cfg = WorldConfig {
        h: WORLD_H,
        w: WORLD_W,
        rules: Rules::default(),
        sun: |_, y| 10 - 10 * y / WORLD_H,
        mineral: |_, y| 10 * y / WORLD_H
    };
    let mut world = World::new(world_cfg);

    let mut pause = true;
    let mut by_step = true;
    let mut cursor_pos = [0.0 as f64; 2];

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);
                const Y_STEP: f64 = WINDOW_H as f64 / WORLD_H as f64;
                const X_STEP: f64 = WINDOW_W as f64 / WORLD_W as f64;
                world.foreach_cell(|x, y, cell| {
                    let rect = [X_STEP * x as f64, Y_STEP * y as f64, X_STEP, Y_STEP];

                    let color = if cell.bot.is_some() {
                        [0.0, 0.0, 0.0, 1.0]
                    } else {
                        [cell.sun as f32/10.0, cell.sun as f32/10.0, cell.mineral as f32/10.0, 1.0]
                    };
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
                        botc::code_packer::to_b32(
                            &botc::compiler::compile(
                            r#"
                            start:
                                ret
                                ret
                                ret
                                ret
                                ret
                                ret
                                eatsun
                                ret
                                ret
                                ret
                                ret
                                ret
                                cmpv EN 1500
                                ret
                                ret
                                ret
                                ret
                                ret
                                ret
                                jle start
                                ret
                                ret
                                ret
                                ret
                                ret
                                ret
                                ret
                                forc front start
                                ret
                                ret
                                ret
                                ret
                                ret
                                ret
                                ret
                                ret
                                ret
                                rot right
                            "#.into())
                            .unwrap(),
                        )
                        .unwrap(),
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