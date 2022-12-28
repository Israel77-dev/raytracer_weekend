use nalgebra::Vector3;

use super::{ray::Ray, Point3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3<f32>,
    pub t: f32,
}

pub trait Hittable {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord>;
}
