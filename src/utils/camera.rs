use nalgebra::Vector3;

use super::{ray::Ray, Point3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        const ASPECT_RATIO: f32 = 16. / 9.;
        const VIEWPORT_HEIGHT: f32 = 2.;
        const VIEWPORT_WIDTH: f32 =
            VIEWPORT_HEIGHT * ASPECT_RATIO;
        const FOCAL_LENGTH: f32 = 1.0;

        let origin: Point3 = Point3::new(0., 0., 0.);
        let horizontal: Point3 =
            Point3::new(VIEWPORT_WIDTH, 0., 0.);
        let vertical: Point3 =
            Point3::new(0., VIEWPORT_HEIGHT, 0.);
        let lower_left_corner: Point3 = origin
            - (horizontal / 2.)
            - (vertical / 2.)
            - Point3::new(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner
                + u * self.horizontal
                + v * self.vertical
                - self.origin,
        )
    }
}
