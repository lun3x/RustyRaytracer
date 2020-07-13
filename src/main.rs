use image::GenericImage;
use cgmath::prelude::*;

type Colour = cgmath::Vector3<u8>;
type Point = cgmath::Vector3<f32>;
type Vector = cgmath::Vector3<f32>;

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

struct Intersection<T: Intersectable> {
    distance: f32,
    object: Box<T>,
}

impl<T:Intersectable> Intersection<T> {
    fn new(distance: f32, object: Box<T>) -> Intersection<T> {
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

struct Scene<T: Intersectable> {
    objects: Vec<Box<T>>,
}

impl<T> Scene<T> where T: Intersectable {
    fn closest_intersection(&self, ray: &Ray) -> Option<Intersection<T>> {
        let mut closest = Intersection {distance:f32::MAX, object: Box::new(Sphere{centre: Point::new(0.0, 0.0, 0.0), radius: 1.0} as dyn Intersectable)};
        for object in self.objects.iter() {
            match object.intersection(ray) {
                Some(distance) => if distance < closest.distance {closest = Intersection {distance, object: object}},
                _ => ()
            };
        }
        return Some(closest);
        // self.objects.iter()
        //             .filter_map(|o| o.intersection(ray).map(|i| Intersection{distance:i.distance2(ray.start), object:o} ))
        //             .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

fn main() {
    println!("Hello, world!");
    let p1 = Point {x:5.0, y:5.0, z:5.0};
    let sphere1 = &Sphere {centre: p1, radius:2.0};
    let triangle1 = &Triangle {v0: p1, v1: p1, v2: p1, normal: p1};
    let objects = vec![sphere1, triangle1];
    // let scene = Scene {
    //     objects: objects
    // };
}
