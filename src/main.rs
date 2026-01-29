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
            position: vec3(10., 0., 10.),
            target: vec3(0., 0., 0.),
            up: vec3(0., 1., 0.),
            ..Default::default()
        });

        // insert object
        draw_text("Hello World", 10.0, 20.0, 20.0, BLACK);
        draw_cube_wires(vec3(0., 0., 0.), vec3(2., 2., 2.), RED);
        draw_grid(30, -1., DARKGRAY, DARKGRAY);
        next_frame().await;
    }
}
