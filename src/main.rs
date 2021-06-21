mod cornell_box;
mod draw;
mod raytracing;
#[cfg(test)]
mod tests;
mod constants;
mod utils;
mod output;

use crate::raytracing::*;
use crate::constants::*;

fn main() {
    let p0 = cgmath::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let camera = Camera::new(p0, 1.0, ASPECT_RATIO, cgmath::Deg(0.0));

    let scene = crate::cornell_box::get_box();
    let scene = crate::cornell_box::get_sphere();

    let mut screen: [u8; 3 * SCREEN_HEIGHT* SCREEN_WIDTH] = [0; 3 * SCREEN_HEIGHT* SCREEN_WIDTH];

    draw::draw_to_screen(&camera, &scene, &mut screen);

    output::write_to_ppm_file(&screen);
}
