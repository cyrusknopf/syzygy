extern crate kiss3d;
extern crate image;
use crate::physics;

use kiss3d::nalgebra::Translation3;
use rand::Rng;
use kiss3d::window::Window;
use physics::Body;

pub fn init (bodies : &Vec<Body>) {
    let mut window = Window::new("Syzygy");
    load_bodies(window, bodies);
}

fn load_body (mut window : Window, body : &Body) -> Window {
    let mut body_node = window.add_sphere(body.radius);
    body_node.set_color(
        rand::thread_rng().gen_range(0..10) as f32 / 10.,
        rand::thread_rng().gen_range(0..10) as f32 / 10.,
        rand::thread_rng().gen_range(0..10) as f32 / 10.
    );
    body_node.set_local_translation(Translation3::new(
            body.position.x,
            body.position.y,
            body.position.z
    ));
    return window;
}

pub fn load_bodies (mut window : Window, bodies : &Vec<Body>) {
    for body in bodies {
        window = load_body(window, body);
    }
}
