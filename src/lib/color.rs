use std::io::Write;

type Color = nalgebra::Vector3<f32>;

pub fn write_color<T: Write>(
    out: &mut T,
    pixel_color: Color,
) {
    let _ = writeln!(
        out,
        "{} {} {}",
        (255.999 * pixel_color.x) as u16,
        (255.999 * pixel_color.y) as u16,
        (255.999 * pixel_color.z) as u16
    );
}
