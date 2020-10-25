mod intersect;
mod objects;
mod scene;
mod visualiser;

use crate::objects::*;
use crate::scene::*;
use crate::visualiser::*;

const RED: [u8; 3] = [255, 0, 0];
const GREEN: [u8; 3] = [0, 255, 0];
const BLUE: [u8; 3] = [0, 0, 255];

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
    let cam_ray = visualiser.create_camera_ray(0, 0);
    println!(
        "camera ray: start=[{},{},{}] dir=[{},{},{}]",
        cam_ray.start.x,
        cam_ray.start.y,
        cam_ray.start.z,
        cam_ray.dir.x,
        cam_ray.dir.y,
        cam_ray.dir.z
    );

    let p1 = Point {
        x: -4.0,
        y: 2.0,
        z: -13.0,
    };
    let p2 = Point {
        x: -2.0,
        y: -2.0,
        z: -7.0,
    };
    let p3 = Point {
        x: 2.0,
        y: 0.0,
        z: -11.0,
    };
    let sphere1 = Object::Sphere(Sphere {
        centre: p1,
        radius: 1.0,
        colour: RED,
    });
    let sphere2 = Object::Sphere(Sphere {
        centre: p2,
        radius: 1.0,
        colour: BLUE,
    });
    let sphere3 = Object::Sphere(Sphere {
        centre: p3,
        radius: 1.0,
        colour: GREEN,
    });
    let v0 = Point {
        x: p1.x,
        y: p1.y,
        z: p1.z + 2.0,
    };
    let v1 = Point {
        x: p2.x,
        y: p2.y,
        z: p2.z + 2.0,
    };
    let v2 = Point {
        x: p3.x,
        y: p3.y,
        z: p3.z + 2.0,
    };
    let triangle1 = Object::Triangle(Triangle::new(p1, p2, p3, [BLUE, GREEN, RED]));
    let objects: Vec<Object> = vec![sphere1, sphere2, sphere3, triangle1];
    let scene = Scene { objects };
    let ray = Ray {
        start: p0,
        dir: Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
    };
    match scene.closest_intersection(&ray) {
        Some(i) => println!("{}m away", i.location.distance),
        _ => (),
    }

    for y in 0..visualiser.screen.height() {
        for x in 0..visualiser.screen.width() {
            let cam_ray = visualiser.create_camera_ray(x, y);
            // println!("camera ray: start=[{},{},{}] dir=[{},{},{}]", cam_ray.start.x, cam_ray.start.y, cam_ray.start.z, cam_ray.dir.x, cam_ray.dir.y, cam_ray.dir.z);
            match scene.closest_intersection(&cam_ray) {
                Some(i) => {
                    visualiser.put_pixel(x, y, i.object.get_colour(i.location.texture_coords))
                }
                _ => visualiser.put_pixel(x, y, [0, 0, 0]),
            }
        }
    }

    visualiser.save();
}
