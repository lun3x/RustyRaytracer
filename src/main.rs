mod cornell_box;
mod intersect;
mod objects;
mod rays;
mod scene;
mod utils;
mod visualiser;

use crate::visualiser::*;
use std::time::{Duration, Instant};

fn main() {
    let start_load = Instant::now();
    println!("Hello, world!");
    let p0 = Point {
        x: 0.0,
        y: 0.0,
        z: 25.0,
    };
    let camera = Camera::new(p0, 1.0,160.0);

    let mut visualiser = Visualiser::new(400, 400, camera);

    let scene = crate::cornell_box::get_scene2();

    let start_render = Instant::now();
    for y in 0..visualiser.screen.height() {
        for x in 0..visualiser.screen.width() {
            let cam_ray = visualiser.create_camera_ray(x, y);
            visualiser.put_pixel(x, y, crate::rays::trace(cam_ray, &scene, 5));
        }
    }
    let render_time = start_render.elapsed().as_millis();
    let total_time = start_load.elapsed().as_millis();
    println!("Render time: {}ms", render_time);
    println!("Total time: {}ms", total_time);

    visualiser.save();
}
