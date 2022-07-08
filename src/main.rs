#![feature(unchecked_math)]

mod fjor;

use std::sync::Mutex;

use fjor::{
    canvas::{Canvas, Color, IntPoint, IntRect},
    sketch::Sketch,
};
use fuss::Simplex;
use once_cell::sync::Lazy;

static SIMPLEX: Lazy<Mutex<Simplex>> = Lazy::new(|| {
    let mut sn = Simplex::new();
    Mutex::new(sn)
});

fn main() {
    let mut driver = Sketch::new(1920, 1080);

    const LOCATION_SCALE: f32 = 0.005;
    const TIME_SCALE: f32 = 0.008;

    driver.on_update = &|canvas: &mut Canvas, frame: usize| {
        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                let red = SIMPLEX.lock().unwrap().noise_3d(
                    x as f32 * LOCATION_SCALE,
                    y as f32 * LOCATION_SCALE,
                    frame as f32 * TIME_SCALE,
                );
                canvas.set_pixel(
                    &IntPoint::new(x as isize, y as isize),
                    &Color::new((255.0 * red) as u8, 0, 0),
                );
            }
        }
    };

    driver.run();
}
