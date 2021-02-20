mod cornell_box;
mod draw;
mod raytracing;
#[cfg(test)]
mod tests;
mod utils;

use crate::draw::*;
use crate::raytracing::*;
use std::time::Instant;

fn main() {
    let start_load = Instant::now();

    let p0 = cgmath::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 25.0,
    };
    let camera = Camera::new(p0, 1.0, cgmath::Deg(0.0));

    let mut visualiser = Visualiser::new(SCREEN_HEIGHT, SCREEN_WIDTH, camera);

    let scene = crate::cornell_box::get_scene();

    let start_render = Instant::now();

    draw::render_scene(visualiser, scene);

    let render_time = start_render.elapsed().as_millis();
    let total_time = start_load.elapsed().as_millis();
    println!("Render time: {}ms", render_time);
    println!("Total time: {}ms", total_time);
}
