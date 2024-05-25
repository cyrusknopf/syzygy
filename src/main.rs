extern crate kiss3d;

mod physics;


use kiss3d::scene::SceneNode;
use rand::Rng;
use kiss3d::nalgebra::Translation3;
use kiss3d::window::Window;
use kiss3d::light::Light;

use physics::Body;

const NUM_PLANETS : i64 = 10;

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
    // Window setup
    let mut window = Window::new("Syzygy");
    // Create some random bodies
    let mut bodies : Vec<Body> = physics::gen_bodies(NUM_PLANETS);
    // Store the 3D objects which represent the bodies
    let mut nodes : Vec<SceneNode> = Vec::new();

    // Initialise each 3D object with a random colour and set its location
    for body in &bodies {
            let mut body_node = window.add_sphere(body.radius as f32);
            body_node.set_color(
                rand::thread_rng().gen_range(0..10) as f32 / 10.,
                rand::thread_rng().gen_range(0..10) as f32 / 10.,
                rand::thread_rng().gen_range(0..10) as f32 / 10.
            );
            body_node.set_local_translation(Translation3::new(
                    body.position.x as f32,
                    body.position.y as f32,
                    body.position.z as f32
            ));
            nodes.push(body_node);
    }

    while window.render() {
        // Update the locations and velocities of all bodies
        bodies = physics::update_all_bodies(&bodies, 0.01, 1000.);

        // Update the position of the corresponding 3D objects
        for i in 0..nodes.len() {
            nodes[i].set_local_translation(Translation3::new(
                    bodies[i].position.x as f32,
                    bodies[i].position.y as f32,
                    bodies[i].position.z as f32
            ));
        }
    }
}
