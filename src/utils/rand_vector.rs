use nalgebra::Vector3;
use rand::{distributions::uniform::SampleRange, Rng};

fn random_vector() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

fn random_vector_between(
    start: f32,
    end: f32,
) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(
        rng.gen_range(start..end),
        rng.gen_range(start..end),
        rng.gen_range(start..end),
    )
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = random_vector_between(0.0, 1.0);
        if p.norm() <= 1.0 {
            return p;
        }
    }
}
