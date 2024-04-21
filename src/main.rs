extern crate kiss3d;

use kiss3d::nalgebra::{Vector3, Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut sphere = window.add_sphere(0.5);

    sphere.set_color(1.0, 0.0, 1.0);

    window.set_light(Light::StickToCamera);

    let mut x_pos = 0.0;


    while window.render() {

        x_pos += 0.01;
        sphere.set_local_translation(Translation3::new(x_pos, 0.0, 0.0));
    }
}
