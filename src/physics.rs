// Adapted from Na Wang's physics engine for Python :salute:

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

const G: f32 = 6.674e-11;

fn distance(loc1 : &Vector3, loc2 : &Vector3) -> f32 {
    let x_dist: f32 = (loc2.x - loc1.x).powi(2);
    let y_dist: f32 = (loc2.y - loc1.y).powi(2);
    let z_dist: f32 = (loc2.z - loc1.z).powi(2);
    return (x_dist + y_dist + z_dist).sqrt();
}

// F = G * (m1 * m2) / r^2
fn force_scalar(mass1 : &i32, mass2 : &i32, d : &f32) -> f32 {
    return G * (mass1 * mass2) as f32 / d.powi(2);
}

fn direction(loc1 : &Vector3, loc2 : &Vector3, d : &f32) -> Vector3 {
    let x_comp : f32 = (loc2.x - loc1.x) / d;
    let y_comp : f32 = (loc2.y - loc1.y) / d;
    let z_comp : f32 = (loc2.z - loc1.z) / d;

    return Vector3 {x: x_comp, y: y_comp, z:z_comp};
}

fn force_vector(f : &f32, direction : &Vector3) -> Vector3 {
    let x_comp : f32 = f * direction.x;
    let y_comp : f32 = f * direction.y;
    let z_comp : f32 = f * direction.z;
    return Vector3 {x: x_comp, y: y_comp, z:z_comp};

}

fn gravitational_force(mass1 : &i32, mass2 : &i32, r1 : &f32, r2 : &f32, loc1 : &Vector3, loc2 : &Vector3) -> Vector3{
    let d : f32 = distance(&loc1, &loc2);
    if (d == 0.0) {
        return Vector3 { x: 0.0, y: 0.0, z: 0.0};
    }
    else {
        let force_scalar : f32 = force_scalar(&mass1, &mass2, &d);
        let direction : Vector3 = direction(&loc1, &loc2, &d);
        return force_vector(&force_scalar, &direction);
    }
}

fn acceleration (force : &f32, mass : &i32) -> f32 {
    return force / (*mass as f32);
}

fn velocity (accel : &f32, time : &f32) -> f32 {
    return accel * time;
}


fn main() {
    let pos1 : Vector3 = Vector3 {x:1.0,y:1.,z:1.};
    let pos2 : Vector3 = Vector3 {x:2.,y:2.,z:2.};
    println!("Distance: {}", distance(&pos1, &pos2));
}
