#![allow(warnings)]

use macroquad::prelude::*;

fn event_listener() -> bool {
    if is_key_pressed(KeyCode::Escape) {
        return false;
    }
    true
}

// create a camera controller

struct CameraController {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
    last_mouse_pos: Option<Vec2>,
}
impl CameraController {
    fn new() -> Self {
        Self {
            position: vec3(0., 0., 0.),
            yaw: -135.0_f32.to_radians(),
            pitch: 30.0_f32.to_radians(),
            distance: 12.0,
            last_mouse_pos: None,
        }
    }

    fn update(&mut self) {
        // Controllo con CTRL + mouse
        let ctrl_pressed = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);

        if ctrl_pressed && is_mouse_button_down(MouseButton::Left) {
            let current_mouse = mouse_position();
            let current_mouse_vec = vec2(current_mouse.0, current_mouse.1);

            if let Some(last_pos) = self.last_mouse_pos {
                let delta = current_mouse_vec - last_pos;

                // Rotazione con mouse (CTRL + trascinamento)
                self.yaw -= delta.x * 0.005;
                self.pitch -= delta.y * 0.005;

                // Limita il pitch per evitare il flip della camera
                self.pitch = self
                    .pitch
                    .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
            }

            self.last_mouse_pos = Some(current_mouse_vec);
        } else {
            self.last_mouse_pos = None;
        }

        // Zoom con rotella del mouse (solo quando CTRL è premuto)
        if ctrl_pressed {
            let wheel = mouse_wheel().1;
            if wheel != 0.0 {
                self.distance -= wheel * 0.5;
                self.distance = self.distance.clamp(3.0, 30.0);
            }
        }
    }
    fn get_camera_position(&self) -> Vec3 {
        let x = self.distance * self.pitch.cos() * self.yaw.cos();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.sin();

        self.position + vec3(x, y, z)
    }

    fn get_camera(&self) -> Camera3D {
        Camera3D {
            position: self.get_camera_position(),
            target: self.position,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        }
    }
}

struct Sfera {
    position: Vec3,
    radius: f32,
    color: Color,
}
impl Sfera {
    fn new() -> Self {
        Self {
            position: vec3(4., 1., 0.),
            radius: 1.0,
            color: GREEN,
        }
    }

    fn update(&mut self) {
        draw_sphere_wires(self.position, self.radius, None, self.color);
    }
}

#[macroquad::main("Test")]
async fn main() {
    let mut camera = CameraController::new();
    let mut sfera1 = Sfera::new();
    loop {
        clear_background(LIGHTGRAY);
        if !event_listener() {
            break;
        }
        camera.update();
        set_camera(&camera.get_camera());
        // setting Camera3D (position, target, up ...)
        //set_camera(&Camera3D {
        //    position: vec3(3., 7., 20.),
        //    target: vec3(0., 0., 0.),
        //    up: vec3(0., 1., 0.),
        //    ..Default::default()
        //});

        // insert object
        draw_sphere_wires(vec3(3., 1., 0.), 1.0, None, BLUE);

        draw_cube_wires(vec3(1., 2., 5.), vec3(1., 1., 1.), RED);
        sfera1.update();

        draw_grid(15, 1., DARKGRAY, YELLOW);

        set_default_camera();
        draw_text("Hello World", 10.0, 20.0, 20.0, BLACK);

        next_frame().await;
    }
}
