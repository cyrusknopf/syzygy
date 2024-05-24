use std::ops::Add;

// Adapted from Na Wang's physics engine for Python 🫡

// Vector struct for defining positions, velocities, etc.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

// Element-wise addition for two vectors
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


// Orbital body struct. Models planets and stars
#[derive(Copy, Clone)]
pub struct Body {
    id: i32,
    mass: i32,
    radius: f32,
    position: Vector,
    velocity: Vector,
}

const NULL_BODY : Body = Body {
    id: -1,
    mass: 0,
    radius: 0.,
    position: Vector {x:0., y: 0., z:0.},
    velocity: Vector {x:0., y: 0., z:0.}
};

// Newton's Gravitational Constant
const G: f32 = 6.674e-11;

// Euclidean Distance between two points in 3D space
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

fn update_positions(bodies: &Vec<Body>, idx: usize) -> Body {
    // The body whose position we are updating
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
        // Summate the forces to find the force exerted on this object by all other objects
            );
        f_sum = f_sum + f;
    }

    // F = ma -> a = f / m
    let accel = Vector {
        x: f_sum.x / this_body.mass as f32,
        y: f_sum.y / this_body.mass as f32,
        z: f_sum.z / this_body.mass as f32,
    };

    // v= u + at
    let new_velocity = Vector {
        x: this_body.velocity.x + accel.x,
        y: this_body.velocity.y + accel.y,
        z: this_body.velocity.z + accel.z,
    };

    // s = ut + 1/2at^2
    let new_pos = Vector {
        x: this_body.position.x + accel.x,
        y: this_body.position.y + accel.y,
        z: this_body.position.z + accel.z,
    };

    let mut new_this_body = this_body;
    new_this_body.position = new_pos;
    new_this_body.velocity = new_velocity;

    return new_this_body;
}

fn model_collisions(bodies: &Vec<Body>, idx1: usize, idx2: usize) -> Vec<Body> {
    let first_body: Body = bodies[idx1];
    let second_body: Body = bodies[idx2];

    let pi : f32 = std::f32::consts::PI;

    let volume1 : f32 = 1.333333333333333 * pi * first_body.radius.powi(3) as f32;
    let volume2 : f32 = 1.333333333333333 * pi * second_body.radius.powi(3) as f32;

    let total_vol : f32 = volume1 + volume2;

    let final_radius : f32 = (total_vol / (pi * 1.333333333333333)).powf(0.33333333333333333);

    // The two coliding bodies merge to form a single body...
    let mut resultant_body : Body = first_body.clone();
    // ... with combined mass
    resultant_body.mass = first_body.mass + second_body.mass;
    // ... with combined radius
    resultant_body.radius = final_radius;

    let resultant_velocity : Vector = Vector {
        x: (first_body.mass as f32 * first_body.velocity.x + second_body.mass as f32 * second_body.velocity.x) / (first_body.mass + second_body.mass) as f32,
        y: (first_body.mass as f32 * first_body.velocity.y + second_body.mass as f32 * second_body.velocity.y) / (first_body.mass + second_body.mass) as f32,
        z: (first_body.mass as f32 * first_body.velocity.z + second_body.mass as f32 * second_body.velocity.z) / (first_body.mass + second_body.mass) as f32
    };
    // ... with combined velocity
    resultant_body.velocity = resultant_velocity;

    let resultant_position : Vector = Vector {
        x: (first_body.position.x + second_body.position.x) / 2.,
        y: (first_body.position.y + second_body.position.y) / 2.,
        z: (first_body.position.z + second_body.position.z) / 2.,
    };
    // ... and combined position.
    resultant_body.position = resultant_position;

    let mut resultant_bodies : Vec<Body> = bodies.clone();
    resultant_bodies[idx2] = NULL_BODY.clone();
    resultant_bodies[idx1] = resultant_body;

    return resultant_bodies;
}

fn update_all_bodies(bodies: &Vec<Body>, timestep : i32, bound : f32) -> Vec<Body> {
    
    // Update positions of all bodies
    let mut resultant_bodies : Vec<Body> = bodies.clone();
    for i in 0..bodies.len() {
        resultant_bodies[i] = update_positions(bodies, i);

        let absolute_x : f32 = resultant_bodies[i].position.x.abs();

        let absolute_y : f32 = resultant_bodies[i].position.y.abs();

        let absolute_z : f32 = resultant_bodies[i].position.z.abs();

        // Check OOB
        if absolute_x > bound {
            resultant_bodies[i] = NULL_BODY.clone();
        }

        if absolute_y > bound {
            resultant_bodies[i] = NULL_BODY.clone();
        }

        if absolute_z > bound {
            resultant_bodies[i] = NULL_BODY.clone();
        }
    }
    return resultant_bodies;
}
