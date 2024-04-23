use std::ops::Add;

// Adapted from Na Wang's physics engine for Python :salute:

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Body {
    id: i32,
    mass: i32,
    radius: f32,
    position: Vector,
    velocity: Vector,
}

const G: f32 = 6.674e-11;

fn distance(loc1: &Vector, loc2: &Vector) -> f32 {
    let x_dist: f32 = (loc2.x - loc1.x).powi(2);
    let y_dist: f32 = (loc2.y - loc1.y).powi(2);
    let z_dist: f32 = (loc2.z - loc1.z).powi(2);
    return (x_dist + y_dist + z_dist).sqrt();
}

// F = G * (m1 * m2) / r^2
fn force_scalar(mass1: &i32, mass2: &i32, d: &f32) -> f32 {
    return G * (mass1 * mass2) as f32 / d.powi(2);
}

fn direction(loc1: &Vector, loc2: &Vector, d: &f32) -> Vector {
    let x_comp: f32 = (loc2.x - loc1.x) / d;
    let y_comp: f32 = (loc2.y - loc1.y) / d;
    let z_comp: f32 = (loc2.z - loc1.z) / d;

    return Vector {
        x: x_comp,
        y: y_comp,
        z: z_comp,
    };
}

fn force_vector(f: &f32, direction: &Vector) -> Vector {
    let x_comp: f32 = f * direction.x;
    let y_comp: f32 = f * direction.y;
    let z_comp: f32 = f * direction.z;
    return Vector {
        x: x_comp,
        y: y_comp,
        z: z_comp,
    };
}

fn gravitational_force(mass1: &i32, mass2: &i32, loc1: &Vector, loc2: &Vector) -> Vector {
    let d: f32 = distance(&loc1, &loc2);
    if d == 0.0 {
        return Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    } else {
        let force_scalar: f32 = force_scalar(&mass1, &mass2, &d);
        let direction: Vector = direction(&loc1, &loc2, &d);
        return force_vector(&force_scalar, &direction);
    }
}

fn acceleration(force: &f32, mass: &i32) -> f32 {
    return force / (*mass as f32);
}

fn velocity(accel: &f32, time: &f32) -> f32 {
    return accel * time;
}

fn update_positions(bodies: &Vec<Body>, idx: usize) -> Vec<Body> {
    let this_body = bodies[idx];
    let mut f_sum = Vector {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    for other_body in bodies.iter() {
        // Don't apply a force to itself
        if other_body.id == this_body.id {
            continue;
        }
        // Calculate the force exerted on this object by the other object
        let f: Vector = gravitational_force(
            &this_body.mass,
            &other_body.mass,
            &this_body.position,
            &other_body.position,
        );
        // Summate the forces to find the force exerted on this object by all other objects
        f_sum = f_sum + f;
    }

    return Vec::new();
}

fn main() {
    let pos1: Vector = Vector {
        x: 1.0,
        y: 1.,
        z: 1.,
    };
    let pos2: Vector = Vector {
        x: 2.,
        y: 2.,
        z: 2.,
    };
    println!("Distance: {}", distance(&pos1, &pos2));
}
