use crate::{canvas::Canvas, tuple::Color};

mod canvas;
mod math_utils;
mod matrix;
mod tuple;

struct Env {
    gravity: tuple::Tuple,
    wind: tuple::Tuple,
}

struct Projectile {
    position: tuple::Tuple,
    velocity: tuple::Tuple,
}

fn tick(env: &Env, proj: &mut Projectile) {
    proj.position = proj.position.add(proj.velocity);
    proj.velocity = proj.velocity.add(env.gravity).add(env.wind)
}

fn main() {
    // println!("Hello, world!");

    let position = tuple::Tuple::new_point(0.0, 1.0, 0.0);
    let velocity = tuple::Tuple::new_vector(1.0, 1.8, 0.0)
        .normalize()
        .multiply(11.25);
    let mut p = Projectile { position, velocity };

    let gravity = tuple::Tuple::new_vector(0.0, -0.1, 0.0);
    let wind = tuple::Tuple::new_vector(-0.01, 0.0, 0.0);
    let e = Env { gravity, wind };

    let mut c = Canvas::new(900, 550);

    loop {
        if p.position.x < 0.0
            || p.position.x as usize >= c.width
            || p.position.y < 0.0
            || p.position.y as usize >= c.height
        {
            break;
        }
        // dbg!(p.position);
        // dbg!(p.velocity);
        c.write_pixel(
            p.position.x as usize,
            c.height - p.position.y as usize,
            Color::new(1.0, 0.0, 0.0),
        );
        tick(&e, &mut p);
    }

    match c.write_to_ppm("output.ppm") {
        Ok(()) => println!("File written successfully."),
        Err(err) => eprintln!("Error: {}", err),
    }
}
