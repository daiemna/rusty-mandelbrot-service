use colorgrad::Gradient;
use image::ImageBuffer;
use image::Rgba;
use log::error;
use std::io;
use std::path::Path;

pub const MAX_COLOR_COUNT: u16 = 1536;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
pub fn write_image<Q>(
    filename: Q,
    pixels: &[u16],
    bounds: (usize, usize),
) -> Result<(), std::io::Error>
where
    Q: AsRef<Path>,
{
    let grad = generate_color_gradient();
    let colors = grad.colors(MAX_COLOR_COUNT as usize);

    let png_img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_fn(bounds.0 as u32, bounds.1 as u32, |x, y| {
            let escape_ind = pixels[(y * bounds.0 as u32 + x) as usize];
            let smooth = (((x * x + y * y) as f32).log2() / 2f32).log2();
            let color_i = ((escape_ind as f32 + 10.0 - smooth).sqrt() * MAX_COLOR_COUNT as f32)
                as usize
                % colors.len();
            let color = &colors[color_i];
            image::Rgba(color.to_rgba8())
        });
    match png_img.save(filename) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error saving image: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}

fn generate_color_gradient() -> Gradient {
    colorgrad::magma()
}
