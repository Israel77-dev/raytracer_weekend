use std::io::Write;

type Color = nalgebra::Vector3<f32>;

pub fn write_color<T: Write>(
    out: &mut T,
    pixel_color: Color,
    samples_per_pixel: u32,
) {
    let scale = 1.0 / (samples_per_pixel as f32);
    let (r, g, b) = (
        pixel_color.x * scale,
        pixel_color.y * scale,
        pixel_color.z * scale,
    );

    let _ = writeln!(
        out,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u16,
        (256.0 * g.clamp(0.0, 0.999)) as u16,
        (256.0 * b.clamp(0.0, 0.999)) as u16
    );
}
