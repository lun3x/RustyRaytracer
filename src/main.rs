mod cornell_box;
mod draw;
mod raytracing;
#[cfg(test)]
mod tests;
mod utils;

use crate::draw::*;
use crate::raytracing::*;
use pixels::Error;

fn main() -> Result<(), Error> {
    let p0 = cgmath::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 25.0,
    };
    let camera = Camera::new(p0, 1.0, cgmath::Deg(0.0));

    let visualiser = Visualiser::new(SCREEN_HEIGHT, SCREEN_WIDTH, camera);

    let scene = crate::cornell_box::get_scene();

    return draw::render_scene(visualiser, scene);
}
