use std::thread::Result;

use nalgebra::Vector3;

use super::{ray::Ray, Point3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_front_face(
        &mut self,
        r: &Ray,
        outward_normal: &Vector3<f32>,
    ) {
        self.front_face =
            r.direction().dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -outward_normal,
        }
    }
}

pub trait Hittable {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord>;
}

pub struct HittableList<T>(Vec<T>);

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn add(&mut self, object: T) {
        self.0.push(object)
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.0.iter() {
            if let Some(a) =
                object.hit(r, t_min, closest_so_far)
            {
                closest_so_far = a.t;
                result = Some(a);
            }
        }

        result
    }
}
