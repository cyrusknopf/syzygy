// Adapted from Na Wang's physics engine for Python :salute:

use kiss3d::nalgebra::ComplexField;

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

const G: f32 = 6.674e-11;

fn distance(loc1 : Vector3, loc2 : Vector3) -> f32 {
    let x_dist: f32 = (loc2.x - loc1.x).powi(2);
    let y_dist: f32 = (loc2.y - loc1.y).powi(2);
    let z_dist: f32 = (loc2.z - loc1.z).powi(2);
    return (x_dist + y_dist + z_dist).sqrt();

}


fn calcGravitationalForce (mass1 : i32, mass2 : i32, r1 : f32, r2 : f32, loc1 : Vector3, loc2 : Vector3) -> Vector3 {
    let d = distance(loc1, loc2);
}
