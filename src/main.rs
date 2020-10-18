mod scene;
mod visualiser;
mod objects;
mod intersect;

use crate::visualiser::*;
use crate::scene::*;
use crate::intersect::*;
use crate::objects::*;

fn main() {
    println!("Hello, world!");
    let p0 = Point {x:0.0, y:0.0, z:0.0};
    let camera = Camera {location: p0, focal_length: 1.0};
    let visualiser = Visualiser::new(50, 50, camera);
    let cam_ray = visualiser.create_camera_ray(0, 0);
    println!("camera ray: start=[{},{},{}] dir=[{},{},{}]", cam_ray.start.x, cam_ray.start.y, cam_ray.start.z, cam_ray.dir.x, cam_ray.dir.y, cam_ray.dir.z);
    
    let p1 = Point {x:0.0, y:0.0, z:5.0};
    let sphere1 = Object::Sphere(Sphere {centre: p1, radius:2.0});
    let triangle1 = Object::Triangle(Triangle {v0: p1, v1: p1, v2: p1, normal: p1});
    let objects: Vec<Object> = vec![sphere1, triangle1];
    let scene = Scene {
        objects: objects
    };
    let ray = Ray {start: p0, dir: Vector{x:1.0, y:1.0, z:1.0}};
    match scene.closest_intersection(&ray) {
        Some(i) => println!("{}m away", i.distance),
        _ => ()
    }
}
