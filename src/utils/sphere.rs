

use super::{hittable::HitRecord, Point3};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl super::hittable::Hittable for Sphere {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let half_b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let delta = half_b * half_b - a * c;

        if delta < 0.0 {
            return None;
        }

        let sqrtd = delta.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b - sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal =
            (p - self.center) / self.radius;

        let mut record = HitRecord {
            p,
            t,
            normal: outward_normal,
            front_face: true,
        };

        record.set_front_face(r, &outward_normal);

        Some(record)
    }
}
