mod lib;
use nalgebra::Vector3;

use crate::lib::{color::write_color, ray::Ray};

type Color = Vector3<f32>;
type Point3 = Vector3<f32>;

fn ray_color(r: Ray) -> Color {
    let unit_direction =
        r.direction() * (1. / (r.direction().norm()));
    let t = 0.5 * (unit_direction.y + 1.);

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

    // Camera
    const VIEWPORT_HEIGHT: f32 = 2.;
    const VIEWPORT_WIDTH: f32 =
        VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f32 = 1.0;

    const ORIGIN: Point3 = Point3::new(0., 0., 0.);
    const HORIZONTAL: Point3 =
        Point3::new(VIEWPORT_WIDTH, 0., 0.);
    const VERTICAL: Point3 =
        Point3::new(0., VIEWPORT_HEIGHT, 0.);

    let LOWER_LEFT_CORNER: Point3 = ORIGIN
        - (HORIZONTAL / 2.)
        - (VERTICAL / 2.)
        - Point3::new(0.0, 0.0, FOCAL_LENGTH);

    // Header
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let v =
                (j as f32) / ((IMAGE_HEIGHT - 1) as f32);

            let r = Ray::new(
                ORIGIN,
                LOWER_LEFT_CORNER
                    + u * HORIZONTAL
                    + v * VERTICAL
                    - ORIGIN,
            );

            let color = ray_color(r);
            write_color(&mut std::io::stdout(), color);
        }
    }
}
