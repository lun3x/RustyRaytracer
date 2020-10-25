mod cornell_box;
mod intersect;
mod objects;
mod rays;
mod scene;
mod visualiser;

use crate::visualiser::*;

fn main() {
    println!("Hello, world!");
    let p0 = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let camera = Camera {
        location: p0,
        focal_length: 1.0,
    };
    let mut visualiser = Visualiser::new(400, 400, camera);

    let scene = crate::cornell_box::get_scene();

    for y in 0..visualiser.screen.height() {
        for x in 0..visualiser.screen.width() {
            let cam_ray = visualiser.create_camera_ray(x, y);
            visualiser.put_pixel(x, y, crate::rays::trace(cam_ray, &scene));
        }
    }

    visualiser.save();
}
