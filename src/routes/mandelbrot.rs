use std::{path::PathBuf, str::FromStr};

use crate::{
    config::Config,
    utils::{pixel_to_point, write_image, MAX_COLOR_COUNT},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use num::Complex;
use serde::{Deserialize, Serialize};

use super::common::ServiceError;

#[derive(Serialize, Deserialize)]
pub struct MandelBrotReq {
    v_size: usize,
    h_size: usize,
    upper_left: [f64; 2],
    lower_right: [f64; 2],
}

#[derive(Serialize, Deserialize, Default)]
pub struct MandelBrotRes {
    image_url: String,
}

pub async fn get_mandelbrot(
    request: web::Json<MandelBrotReq>,
    data: web::Data<Config>,
) -> Result<HttpResponse, ServiceError> {
    let mut pixels = vec![0; request.v_size * request.h_size];
    let bounds = (request.v_size, request.h_size);
    let u_left = Complex {
        re: request.upper_left[0],
        im: request.upper_left[1],
    };
    let l_right = Complex {
        re: request.lower_right[0],
        im: request.lower_right[1],
    };
    render(&mut pixels, bounds, u_left, l_right);

    let now = Utc::now().format("%Y%m%d_%H%M%S_%3f");
    let mut file_path: PathBuf = PathBuf::from_str(data.render_dir.as_ref()).unwrap();
    let filename = format!("mandalbrot_{}.png", now);
    file_path.push::<&str>(&filename);

    if let Err(_) = write_image(file_path.as_path(), &pixels, bounds) {
        return Err(ServiceError::FileNotCreated);
    }
    let res: MandelBrotRes = MandelBrotRes {
        image_url: format!("http://{}:{}/files/{}", data.host, data.port, filename),
    };
    return Ok(HttpResponse::Ok().json(res));
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// # Arguments
///
/// * `pixels` - image pixels to be filled.
/// * `bounds` - tuple of size of image.
/// * `upper_left` - upper left edge of the image.
/// * `lower_right` - lower right edge of the image.
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u16],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point: Complex<f64> =
                pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, MAX_COLOR_COUNT as usize) {
                None => 0,
                Some(count) => MAX_COLOR_COUNT - (count as u16),
            };
        }
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2 centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

#[test]
fn test_escape_time() {
    if let Some(iter) = escape_time(Complex { re: 1.0, im: 1.0 }, 10) {
        assert_eq!(iter, 2);
    }
    if let Some(iter) = escape_time(Complex { re: -1.0, im: -1.0 }, 10) {
        assert_eq!(iter, 3);
    }
    if let Some(iter) = escape_time(Complex { re: -1.0, im: 0.0 }, 100) {
        assert_eq!(iter, 2);
    }
    if let Some(iter) = escape_time(Complex { re: 1.0, im: -1.0 }, 10) {
        assert_eq!(iter, 2);
    }
}
