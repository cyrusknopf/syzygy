extern crate kiss3d;

mod physics;
mod graphics;

use kiss3d::scene::SceneNode;
use rand::Rng;
use kiss3d::nalgebra::Translation3;
use kiss3d::window::Window;
use kiss3d::light::Light;

use physics::Body;

const NUM_PLANETS : i32 = 5;

pub struct Body3D {
    pub body: Body,
    pub node: SceneNode
}

fn basic_example() {
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

fn main() {
    let mut window = Window::new("Syzygy");
    let mut bodies : Vec<Body> = physics::gen_bodies(NUM_PLANETS);
    let mut nodes : Vec<SceneNode> = Vec::new();

    for body in &bodies {
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
            nodes.push(body_node);
    }

    while window.render() {
        bodies = physics::update_all_bodies(&bodies, 0.001, 500.);

        for i in 0..nodes.len() {
            nodes[i].set_local_translation(Translation3::new(
                    bodies[i].position.x,
                    bodies[i].position.y,
                    bodies[i].position.z
            ));
        }
    }
}
