use std::ops::Add;
use rand::Rng;

// Adapted from Na Wang's physics engine for Python 🫡

// Vector struct for defining positions, velocities, etc.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
    pub id: i64,
    pub mass: i64,
    pub radius: f64,
    pub position: Vector,
    pub velocity: Vector
}

const NULL_BODY : Body = Body {
    id: -1,
    mass: 0,
    radius: 0.,
    position: Vector {x:0., y: 0., z:0.},
    velocity: Vector {x:0., y: 0., z:0.}
};

// My Newton's Gravitational Constant
const G: f64 = 6.674e-5;

const SOLAR : i64 = -2;

// Euclidean Distance between two points in 3D space
fn distance(loc1: &Vector, loc2: &Vector) -> f64 {
    let x_dist: f64 = (loc2.x - loc1.x).powi(2);
    let y_dist: f64 = (loc2.y - loc1.y).powi(2);
    let z_dist: f64 = (loc2.z - loc1.z).powi(2);
    return (x_dist + y_dist + z_dist).sqrt();
}

// F = G * (m1 * m2) / r^2
fn force_scalar(mass1: &i64, mass2: &i64, d: &f64) -> f64 {
    return G * (mass1 * mass2) as f64 / d.powi(2);
}

fn direction(loc1: &Vector, loc2: &Vector, d: &f64) -> Vector {
    let x_comp: f64 = (loc2.x - loc1.x) / d;
    let y_comp: f64 = (loc2.y - loc1.y) / d;
    let z_comp: f64 = (loc2.z - loc1.z) / d;

    return Vector {
        x: x_comp,
        y: y_comp,
        z: z_comp,
    };
}

fn force_vector(f: &f64, direction: &Vector) -> Vector {
    let x_comp: f64 = f * direction.x;
    let y_comp: f64 = f * direction.y;
    let z_comp: f64 = f * direction.z;
    return Vector {
        x: x_comp,
        y: y_comp,
        z: z_comp,
    };
}

fn gravitational_force(mass1: &i64, mass2: &i64, loc1: &Vector, loc2: &Vector) -> Vector {
    let d: f64 = distance(&loc1, &loc2);
    if d == 0.0 {
        return Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    } else {
        let force_scalar: f64 = force_scalar(&mass1, &mass2, &d);
        let direction: Vector = direction(&loc1, &loc2, &d);
        return force_vector(&force_scalar, &direction);
    }
}

fn acceleration(force: &f64, mass: &i64) -> f64 {
    return force / (*mass as f64);
}

fn velocity(accel: &f64, time: &f64) -> f64 {
    return accel * time;
}

fn update_positions(bodies: &Vec<Body>, idx: usize, timestep : f64) -> Body {
    // The body whose position we are updating
    let this_body = bodies[idx];
    if this_body.id == SOLAR { return this_body; }
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
        print!("{},{},{} force exerted onto {}\n",f_sum.x, f_sum.y, f_sum.z , idx);
    }

    // F = ma -> a = f / m
    let accel = Vector {
        x: f_sum.x / this_body.mass as f64,
        y: f_sum.y / this_body.mass as f64,
        z: f_sum.z / this_body.mass as f64,
    };

    // v= u + at
    let new_velocity = Vector {
        x: this_body.velocity.x + accel.x,
        y: this_body.velocity.y + accel.y,
        z: this_body.velocity.z + accel.z,
    };

    // s = ut + 1/2at^2
    let new_pos = Vector {
        x: this_body.position.x + this_body.velocity.x * timestep + 0.5 * accel.x * timestep.powi(2),
        y: this_body.position.y + this_body.velocity.y * timestep + 0.5 * accel.y * timestep.powi(2),
        z: this_body.position.z + this_body.velocity.z * timestep + 0.5 * accel.z * timestep.powi(2),
    };


    Body {
        id: this_body.id,
        mass: this_body.mass,
        radius: this_body.radius,
        position: new_pos,
        velocity: new_velocity,
    }

}

fn model_collisions(bodies: &Vec<Body>, idx1: usize, idx2: usize) -> Vec<Body> {
    let first_body: Body = bodies[idx1];
    let second_body: Body = bodies[idx2];

    let pi : f64 = std::f64::consts::PI;

    let volume1 : f64 = 1.333333333333333 * pi * first_body.radius.powi(3) as f64;
    let volume2 : f64 = 1.333333333333333 * pi * second_body.radius.powi(3) as f64;

    let total_vol : f64 = volume1 + volume2;

    let final_radius : f64 = (total_vol / (pi * 1.333333333333333)).powf(0.33333333333333333);

    // The two coliding bodies merge to form a single body...
    let mut resultant_body : Body = first_body.clone();
    // ... with combined mass
    resultant_body.mass = first_body.mass + second_body.mass;
    // ... with combined radius
    resultant_body.radius = final_radius;

    let resultant_velocity : Vector = Vector {
        x: (first_body.mass as f64 * first_body.velocity.x
            + second_body.mass as f64 * second_body.velocity.x) / (first_body.mass + second_body.mass) as f64,

        y: (first_body.mass as f64 * first_body.velocity.y
            + second_body.mass as f64 * second_body.velocity.y) / (first_body.mass + second_body.mass) as f64,

        z: (first_body.mass as f64 * first_body.velocity.z
            + second_body.mass as f64 * second_body.velocity.z) / (first_body.mass + second_body.mass) as f64
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

pub fn update_all_bodies(bodies: &Vec<Body>, timestep : f64, bound : f64) -> Vec<Body> {
    // Update positions of all bodies
    let mut resultant_bodies : Vec<Body> = bodies.clone();
    for i in 0..bodies.len() {
        if bodies[i].id == -1 {
            continue;
        }
        resultant_bodies[i] = update_positions(&bodies, i, timestep);

        let absolute_x : f64 = resultant_bodies[i].position.x.abs();

        let absolute_y : f64 = resultant_bodies[i].position.y.abs();

        let absolute_z : f64 = resultant_bodies[i].position.z.abs();

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

        // Check against all other bodies if there is a collision
        for j in 0..resultant_bodies.len() {
            // Don't check against self
            if bodies[i].id == bodies[j].id {
                continue;
            }
            // If the distance between two bodies is less than the sum of their radi, there is a
            // collision. Model accordingly
            if distance(&bodies[i].position, &bodies[j].position) <
                bodies[i].radius + bodies[j].radius {
                    resultant_bodies[j] = NULL_BODY.clone();
            }
        }
    }
    return resultant_bodies;
}

fn gen_body(id : i64) -> Body {
    let rand_pos = Vector {
        x: rand::thread_rng().gen_range(-50..50) as f64,
        y: rand::thread_rng().gen_range(-50..50) as f64,
        z: rand::thread_rng().gen_range(-50..50) as f64
        };

    let rand_vel = Vector {
        x: rand::thread_rng().gen_range(-10..10) as f64,
        y: rand::thread_rng().gen_range(-10..10) as f64,
        z: rand::thread_rng().gen_range(-10..10) as f64
    };

    let body = Body {
        id: id,
        mass: rand::thread_rng().gen_range(1..100001),
        radius: rand::thread_rng().gen_range(0.001..5.) as f64,
        position: rand_pos,
        velocity: rand_vel
    };

    return body;
}

pub fn gen_bodies(n : i64) -> Vec<Body> {
    let mut bodies = Vec::new();
    for i in 0..n {
        bodies.push(gen_body(i));
    }
    let sun = gen_solar(10., 5000000);
    bodies.push(sun);

    return bodies;
}

pub fn gen_solar (radius : f64, mass : i64) -> Body {
    let sun =Body {
        id: SOLAR,
        mass: mass,
        radius: radius,
        position: Vector {x:0., y:0. ,z:0.},
        velocity: Vector {x:0., y:0. ,z:0.},
    };
    return sun;
}
