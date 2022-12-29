use std::f32::INFINITY;

use nalgebra::Vector3;

use crate::utils::{
    rand_vector::random_in_unit_sphere, Color,
};

use super::hittable::Hittable;

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(
        origin: Vector3<f32>,
        direction: Vector3<f32>,
    ) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }
}

pub fn ray_color<T: Hittable>(
    r: &Ray,
    world: &T,
    depth: u32,
) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hr) = world.hit(r, 0.001, INFINITY) {
        let target =
            hr.p + hr.normal + random_in_unit_sphere();

        return 0.5
            * ray_color(
                &Ray::new(hr.p, target - hr.p),
                world,
                depth - 1,
            );
    }

    let unit_direction =
        r.direction() * (1. / (r.direction().norm()));
    let t = 0.5 * (unit_direction.y + 1.0);

    const WHITE: Color = Color::new(1.0, 1.0, 1.0);
    const BLUE: Color = Color::new(0.5, 0.7, 1.0);

    WHITE.lerp(&BLUE, t)
}
