use cgmath::prelude::*;
use crate::objects::*;
use crate::visualiser::*;

pub struct Intersection<'a> {
    pub distance: f32,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f32, object: &'a Object) -> Intersection {
        Intersection {
            distance,
            object
        }
    }
}

pub trait Intersectable {
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