mod utils;

use rand::{distributions::Uniform, Rng};

use crate::utils::{
    camera::Camera, color::write_color,
    hittable::HittableList, ray::ray_color, sphere::Sphere,
    Color, Point3,
};

fn main() {
    // Image definition
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 =
        (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

    // Adjusts
    const SAMPLES_PER_PIXEL: u32 = 80;
    const MAX_DEPTH: u32 = 50;

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

    // RNG
    let mut rng = rand::thread_rng();
    let offset_dist: Uniform<f32> = Uniform::new(0.0, 1.0);

    // Header
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32
                    + rng.sample(offset_dist))
                    / ((IMAGE_WIDTH - 1) as f32);

                let v = (j as f32
                    + rng.sample(offset_dist))
                    / ((IMAGE_HEIGHT - 1) as f32);

                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world, MAX_DEPTH);
            }

            write_color(
                &mut std::io::stdout(),
                color,
                SAMPLES_PER_PIXEL,
            );
        }
    }
}
