mod utils;
use std::f32::INFINITY;

use nalgebra::Vector3;
use utils::hittable;

use crate::utils::{
    camera::Camera,
    color::write_color,
    hittable::{Hittable, HittableList},
    ray::Ray,
    sphere::Sphere,
    Point3,
};

type Color = Vector3<f32>;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(hr) = world.hit(r, 0.0, INFINITY) {
        let n = hr.normal;

        return 0.5 * (n + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction =
        r.direction() * (1. / (r.direction().norm()));
    let t = 0.5 * (unit_direction.y + 1.0);

    const WHITE: Color = Color::new(1.0, 1.0, 1.0);
    const BLUE: Color = Color::new(0.5, 0.7, 1.0);

    WHITE.lerp(&BLUE, t)
}

fn main() {
    // Image size
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 =
        (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

    // World
    let mut world: HittableList<Sphere> =
        HittableList::new();
    world
        .add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    ));

    // Camera
    let camera = Camera::new();

    // Header
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let v =
                (j as f32) / ((IMAGE_HEIGHT - 1) as f32);

            let r = camera.get_ray(u, v);

            let color = ray_color(&r, &world);
            write_color(&mut std::io::stdout(), color);
        }
    }
}
