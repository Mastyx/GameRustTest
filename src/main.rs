#![allow(warnings)]

use macroquad::prelude::*;

fn event_listener() -> bool {
    if is_key_pressed(KeyCode::Escape) {
        return false;
    }
    true
}

#[macroquad::main("")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        if !event_listener() {
            break;
        }
        // setting Camera3D (position, target, up ...)
        set_camera(&Camera3D {
            position: vec3(0., 5., 20.),
            target: vec3(0., 0., 0.),
            up: vec3(0., 1., 0.),
            ..Default::default()
        });

        // insert object
        draw_text("Hello World", 10.0, 20.0, 20.0, BLACK);

        draw_cube_wires(vec3(1., 3., 5.), vec3(1., 1., 1.), RED);

        draw_grid(15, 1., DARKGRAY, YELLOW);
        next_frame().await;
    }
}
