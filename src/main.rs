use image::GenericImage;
use cgmath::prelude::*;

type Colour = cgmath::Vector3<u8>;
type Point = cgmath::Vector3<f32>;
type Vector = cgmath::Vector3<f32>;

enum Object {
    Triangle(Triangle),
    Sphere(Sphere),
}

struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub normal: Vector,
}

struct Sphere {
    pub centre: Point,
    pub radius: f32,
}

struct Ray {
    start: Point,
    dir: Vector,
}

trait Intersectable {
    fn intersection(&self, ray: &Ray) -> Option<f32>;
}

impl Intersectable for Object {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        match *self {
            Object::Sphere(ref s) => s.intersection(ray),
            Object::Triangle(ref t) => t.intersection(ray),
        }
    }
}

struct Intersection<'a> {
    distance: f32,
    object: &'a Object,
}

impl<'a> Intersection<'a> {
    fn new(distance: f32, object: &'a Object) -> Intersection {
        Intersection {
            distance,
            object
        }
    }
}

impl Intersectable for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        let to_centre: Vector = self.centre - ray.start;
        // Right angle triangle side lengths, with to_centre as the hypotenuse
        let adjacent: f32 = ray.dir.dot(to_centre);
        let opposite_squared: f32 = to_centre.dot(to_centre) - (adjacent * adjacent);
        let radius_squared: f32 = self.radius * self.radius;

        // No intersect if opposite is greater than radius^2
        if opposite_squared > radius_squared {
            return None;
        }

        // Length of chord (section of ray that is intersecting)
        let half_chord: f32 = (radius_squared - opposite_squared).sqrt();
        // Intersection distances
        let isect0 = adjacent - half_chord;
        let isect1 = adjacent + half_chord;
        
        // Dont return intersections behind camera
        if isect0.is_sign_negative() && isect1.is_sign_negative() {
            return None;
        }

        return Some(if isect0 < isect1 {isect0} else {isect1});
    }
}

impl Intersectable for Triangle {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        return None
    }
}

struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    fn closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_dist = f32::MAX;
        let mut closest_isec: Option<Intersection> = None;
        for object in self.objects.iter() {
            match object.intersection(ray) {
                Some(distance) => if distance < closest_dist {
                    closest_isec = Some(Intersection::new(distance, object));
                    closest_dist = distance;
                },
                _ => ()
            }
        }
        return closest_isec;
    }
}

fn main() {
    println!("Hello, world!");
    let p0 = Point {x:0.0, y:0.0, z:0.0};
    let p1 = Point {x:5.0, y:5.0, z:5.0};
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
